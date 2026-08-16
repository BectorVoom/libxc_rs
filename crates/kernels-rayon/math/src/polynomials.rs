//! Polynomial and rational function evaluation using Horner's method.
//!
//! Used internally by the erf implementation and by GGA/MGGA functionals.
//! Generic over `` to support both f64 and f32.


/// Evaluate a polynomial using Horner's method.
///
/// Coefficients are ordered highest-degree first:
/// `coeffs = [a_n, a_{n-1}, ..., a_1, a_0]`
/// computes `a_n * x^n + a_{n-1} * x^{n-1} + ... + a_1 * x + a_0`
///
/// Horner form: `((a_n * x + a_{n-1}) * x + ...) * x + a_0`
pub fn poly_eval(x: f64, coeffs: &[f64], n: usize) -> f64 {
    let mut result = coeffs[0usize];
    let mut i = 1usize;
    while i < n {
        result = result * x + coeffs[i];
        i += 1;
    }
    result
}

/// Evaluate a rational function P(x)/Q(x) using Horner's method for both.
///
/// Both `p` and `q` coefficient arrays are highest-degree first.
pub fn rational_eval(
    x: f64,
    p: &[f64],
    q: &[f64],
    np: usize,
    nq: usize,
) -> f64 {
    poly_eval(x, p, np) / poly_eval(x, q, nq)
}
