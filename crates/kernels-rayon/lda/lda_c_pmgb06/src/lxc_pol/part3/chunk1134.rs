//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1134/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1134(t1444: f64, t4605: f64, t493: f64, t834: f64, t9354: f64, t1972: f64, t3243: f64, t3247: f64, t842: f64, t3250: f64, t5175: f64, t5179: f64) -> (f64, f64, f64, f64, f64) {
    let t13477 = t1444 * t4605 / 15.0_f64;
    let t13480 = t493 * t9354 * t834 / 45.0_f64;
    let t13482 = t1972 * t3243 / 45.0_f64;
    let t13483 = t3247 * t842;
    let t13486 = 8.0_f64 / 81.0_f64 * t493 * t13483 * t3250;
    let t13489 = 2.0_f64 / 5.0_f64 * t493 * t5179 * t5175;
    (t13477, t13480, t13482, t13486, t13489)
}
