use std::cmp::Ordering;
use std::io::stdin;

fn guess_number(secret_number: i32) -> i32 {
    let mut guess = 4;
    let diff = guess / 2;

    loop {
        println!("Guessing {}", guess);

        match secret_number.cmp(&guess) {
            Ordering::Greater => {
                guess += diff;
            }
            Ordering::Less => {
                guess -= diff / 2;
            }
            Ordering::Equal => return guess,
        }
    }
}

fn main() {
    println!("Welcome to the reverse guessing game!");
    println!("Please guess a number between 1 and 100");

    let mut secret_number = String::new();

    stdin()
        .read_line(&mut secret_number)
        .expect("Failed to read user input");

    let secret_number: i32 = secret_number.trim().parse().expect("Not a valid number");

    println!("Your secret number id {}", guess_number(secret_number));
}

#[cfg(test)]
mod tests {
    use super::guess_number;

    #[test]
    fn test_guess_number() {
        for num in -1000..1000 {
            let result = guess_number(num);
            assert_eq!(num, result);
        }
    }
}
