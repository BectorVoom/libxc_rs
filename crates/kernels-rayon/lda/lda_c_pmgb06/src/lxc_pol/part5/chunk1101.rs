//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1101/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1101(t161: f64, t489: f64, t7807: f64, t1915: f64, t19349: f64, t493: f64, t1919: f64, t19354: f64, t1972: f64, t6770: f64, t1380: f64, t529: f64, t7612: f64) -> (f64, f64, f64, f64, f64) {
    let t20234 = t161 * t489 * t7807;
    let t20235 = 2.0_f64 / 15.0_f64 * t20234;
    let t20238 = 2.0_f64 / 45.0_f64 * t493 * t1915 * t19349;
    let t20241 = t493 * t1919 * t19354 / 27.0_f64;
    let t20243 = t1972 * t6770 / 9.0_f64;
    let t20247 = t493 * t1380 * t7612 * t529 / 45.0_f64;
    (t20235, t20238, t20241, t20243, t20247)
}
