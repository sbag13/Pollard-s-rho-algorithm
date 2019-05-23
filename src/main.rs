use openssl::bn::BigNum;
use openssl::bn::BigNumContext;
use openssl::rsa::Rsa;

static PEM : &str = "-----BEGIN RSA PRIVATE KEY-----
MIIBOQIBAAJBALWWyTbGlyMdHnHBpXzVeQAQks6wN4Fz4LRKiIDVokz1yFQz/kEG
kpDPCSkm0PhSqsGtbW81NJ6Yai00wHt/GCkCAQMCQHkPMM8vD2y+FEvWblM4+1VgYd8gJQD36yLcWwCObDNNZTpcAeDKVdzivXnw89DY5pFBBLOrLTKffSGVyviwAOMC
IQDoz24m1pRqocB9FMru41swAyI7b1WN9WqL03SaPdRpHwIhAMetO9SZQqcjum/dcnRbsiDkqaryXuLdPqKnX3XIoq23AiEAmzT0GeRi8cEq/g3cn0I8yqzBfPTjs/jx
soz4ZtPi8L8CIQCFHifjENcaF9Gf6Pb4PSFrQxvHTD9B6NRsb5T5MGxzzwIgS5xZJKclJsd+UjKEbziUG0xGojjxdBz5ySDPT4ArljY=
-----END RSA PRIVATE KEY-----";

fn main() {
    let mut q_minus_1 = BigNum::new().unwrap();
    let mut p_minus_1 = BigNum::new().unwrap();
    match Rsa::private_key_from_pem(PEM.as_ref()) {
        Ok(key) => {
            println!("n: {:?}", key.n());
            println!("p: {:?}", key.p());
            println!("q: {:?}", key.q());

            q_minus_1 = key.q().unwrap().to_owned().unwrap();
            q_minus_1.sub_word(1).unwrap();
            p_minus_1 = key.p().unwrap().to_owned().unwrap();
            p_minus_1.sub_word(1).unwrap();
        }
        Err(e) => println!("{:?}", e),
    }
    
    println!("p-1 {:?}", p_minus_1);
    println!("q-1 {:?}", q_minus_1);

    let mut ctx = BigNumContext::new().unwrap();

    let mut tmp_n = q_minus_1;
    let mut res = pollard(&tmp_n, &mut ctx);
    while res.is_some() {
        println!("divisor {:?}", res);
        let mut divided = BigNum::new().unwrap();
        divided.checked_div(&tmp_n, &res.unwrap(), &mut ctx).unwrap();
        println!("n {:?}", divided);
        res = pollard(&divided, &mut ctx);
        tmp_n = divided;
    }
    println!("{:?}", tmp_n);
}

fn g(x: &BigNum, m: &BigNum, ctx: &mut BigNumContext) -> BigNum {
    let mut sqr = BigNum::new().unwrap();
    sqr.checked_mul(x, x, ctx).unwrap();
    let mut res = BigNum::new().unwrap();
    res.nnmod(&sqr, m, ctx).unwrap();
    res
}

fn pollard(n: &BigNum, ctx: &mut BigNumContext) -> Option<BigNum> {
    let mut x = BigNum::from_u32(2).unwrap();
    let mut y = BigNum::from_u32(2).unwrap();
    let mut d = BigNum::from_u32(1).unwrap();
    while d == BigNum::from_u32(1).unwrap() {
        x = g(&x, &n, ctx);
        let tmp = g(&y, &n, ctx);
        y = g(&tmp, &n, ctx);
        let mut abs = BigNum::new().unwrap();
        if x > y {
            abs.checked_sub(&x, &y).unwrap();
        } else {
            abs.checked_sub(&y, &x).unwrap();
        }
        d.gcd(&abs, &n, ctx).unwrap();
    }

    if d == *n {
        return None;
    } else {
        return Some(d);
    }
}
