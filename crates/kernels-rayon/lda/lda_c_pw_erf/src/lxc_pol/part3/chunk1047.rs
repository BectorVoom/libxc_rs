//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1047/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1047(t1943: f64, t2973: f64, t2017: f64, t571: f64, t1381: f64, t1954: f64, t4841: f64, t1472: f64, t4843: f64, t2014: f64, t3727: f64, t4807: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12264 = t1943 * t2973;
    let t12267 = 4.0_f64 / 27.0_f64 * t571 * t2017 * t12264;
    let t12271 = 8.0_f64 / 15.0_f64 * t571 * t4841 * t1954 * t1381;
    let t12273 = 16.0_f64 / 15.0_f64 * t1472 * t4843;
    let t12275 = 8.0_f64 / 15.0_f64 * t3727 * t2014;
    let t12277 = 8.0_f64 / 15.0_f64 * t1472 * t4807;
    (t12264, t12267, t12271, t12273, t12275, t12277)
}
