//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 906/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk906(t30817: f64, t7836: f64, t1190: f64, t30540: f64, t30219: f64, t7867: f64, t7871: f64, t1165: f64, t3346: f64, t604: f64, t7493: f64, t2070: f64, t30792: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30840 = t30817 * t7836;
    let t30844 = t30540 * t1190;
    let t30846 = t30219 * t7867;
    let t30848 = t30219 * t7871;
    let t30852 = t7493 * t1165 * t604 * t3346;
    let t30854 = t30792 * t2070;
    (t30840, t30844, t30846, t30848, t30852, t30854)
}
