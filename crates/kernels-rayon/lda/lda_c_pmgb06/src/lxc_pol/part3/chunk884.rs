//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 884/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk884(t3112: f64, t350: f64, t139: f64, t1767: f64, t134: f64, t138: f64, t1537: f64, t947: f64, t1527: f64, t3117: f64, t3094: f64, t3259: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9151 = t350 * t3112;
    let t9175 = t1767 * t139;
    let t9177 = t138 * t9175 * t134;
    let t9178 = 0.01959135802469136_f64 * t9177;
    let t9179 = t947 * t1537;
    let t9181 = t947 * t1527;
    let t9184 = t350 * t3117;
    let t9186 = t350 * t3094;
    let t9188 = t139 * t3259;
    (t9151, t9175, t9177, t9178, t9179, t9181, t9184, t9186, t9188)
}
