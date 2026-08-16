//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 567/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk567(t2998: f64, t400: f64, t1: f64, t960: f64, t397: f64, t1073: f64, t1081: f64, t1124: f64, t119: f64, t84: f64, t395: f64, t1035: f64, t339: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2999 = t400 * t2998;
    let t3000 = 51.94726769812759_f64 * t2999;
    let t3001 = t960 * t1;
    let t3002 = t3001 * t397;
    let t3003 = 0.0005493466511025948_f64 * t3002;
    let t3004 = t1073 * t1081;
    let t3005 = 0.0007324622014701264_f64 * t3004;
    let t3007 = t119 * t1124 * t84;
    let t3008 = t395 * t3007;
    let t3009 = 0.0005696928233656539_f64 * t3008;
    let t3010 = t339 * t1035;
    (t2999, t3000, t3001, t3002, t3003, t3004, t3005, t3007, t3008, t3009, t3010)
}
