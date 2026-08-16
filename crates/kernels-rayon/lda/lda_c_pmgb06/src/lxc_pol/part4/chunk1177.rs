//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1177/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1177(t5044: f64, t831: f64, t11777: f64, t9242: f64, t9259: f64, t5302: f64, t802: f64, t9267: f64, t9269: f64, t11792: f64, t12261: f64, t161: f64, t166: f64, t851: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15467 = t831 * t5044;
    let t15468 = 2.0_f64 / 135.0_f64 * t15467;
    let t15469 = 8.0_f64 / 405.0_f64 * t11777;
    let t15470 = t9242 / 135.0_f64;
    let t15471 = t9259 / 135.0_f64;
    let t15472 = t802 * t5302;
    let t15473 = 4.0_f64 / 45.0_f64 * t15472;
    let t15474 = 4.0_f64 / 405.0_f64 * t9267;
    let t15475 = 4.0_f64 / 405.0_f64 * t9269;
    let t15476 = 4.0_f64 / 45.0_f64 * t11792;
    let t15480 = t161 * t166 * t12261 * t851 / 15.0_f64;
    (t15468, t15469, t15470, t15471, t15473, t15474, t15475, t15476, t15480)
}
