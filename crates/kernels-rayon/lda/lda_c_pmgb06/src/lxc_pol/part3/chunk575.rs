//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 575/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk575(t132: f64, t3076: f64, t134: f64, t138: f64, t2897: f64, t455: f64, t947: f64, t1527: f64, t350: f64, t1533: f64, t1537: f64, t139: f64, t1435: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3077 = t132 * t3076;
    let t3078 = t3077 / 15.0_f64;
    let t3080 = t138 * t2897 * t134;
    let t3081 = 0.005877407407407408_f64 * t3080;
    let t3082 = t947 * t455;
    let t3084 = t350 * t1527;
    let t3086 = t350 * t1533;
    let t3088 = t350 * t1537;
    let t3090 = t139 * t1435;
    (t3077, t3078, t3080, t3081, t3082, t3084, t3086, t3088, t3090)
}
