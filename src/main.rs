/*
 * Rustの型（配列）。
 * CreatedAt: 2019-05-31
 */
fn main() {
    let a = [1, 3, 5];
    println!("{:?}", a);

    println!("{}", a[0]);
    println!("{}", a[1]);
    println!("{}", a[2]);
//    println!("{}", a[3]); // error: index out of bounds: the len is 3 but the index is 3

//    a[2] = 55; // error[E0594]: cannot assign to indexed content `a[..]` of immutable binding
//    println!("{}", a[2]);

    let mut a = [0, 2, 4];
    println!("{:?}", a);
    a[2] = 444;
    println!("{}", a[2]);
}

