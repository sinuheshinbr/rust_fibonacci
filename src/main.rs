use std::io;

fn main() {
    println!("Generate the nth fibonacci number");

    println!("Type n");

    let mut n = String::new();

    io::stdin().read_line(&mut n).expect("Failed to read line");

    let n: u32 = n.trim().parse().expect("Value is not a valid number");

    let last_digit = n % 10;

    let ordinal = match last_digit {
        _ if last_digit == 1 => "st",
        _ if last_digit == 2 => "nd",
        _ if last_digit == 3 => "rd",
        _ => "th",
    };

    match n {
        _ if n < 1 => println!("Invalid number"),
        _ => println!("The {}{} fibonacci number is {}", n, ordinal, fib(n - 1)),
    }
}

fn fib(n: u32) -> u32 {
    if n <= 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    fib(n - 1) + fib(n - 2)
}
