//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 586/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk586(t128: f64, t3251: f64, t10: f64, t1686: f64, t19: f64, t436: f64, t299: f64, t411: f64, t732: f64, t155: f64, t1568: f64, t119: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3252 = t128 * t3251;
    let t3253 = t10 * t3252;
    let t3257 = t1686 * t436 * t19;
    let t3259 = t732 * t299 * t411;
    let t3260 = t3257 * t3259;
    let t3262 = t155 * t1568;
    let t3263 = t119 * t3262;
    (t3252, t3253, t3257, t3259, t3260, t3262, t3263)
}
