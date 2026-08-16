//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1008/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1008(t9177: f64, t1537: f64, t947: f64, t1527: f64, t139: f64, t3259: f64, t1437: f64, t1830: f64, t455: f64, t1530: f64, t1490: f64, t1554: f64, t161: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9178 = 0.01959135802469136_f64 * t9177;
    let t9179 = t947 * t1537;
    let t9181 = t947 * t1527;
    let t9188 = t139 * t3259;
    let t9189 = t1437 * t1437;
    let t9190 = 1.0_f64 / t9189;
    let t9215 = t1830 * t455;
    let t9220 = 1.0_f64 / t1437 / t1530;
    let t9242 = t161 * t1554 * t1490;
    (t9178, t9179, t9181, t9188, t9190, t9215, t9220, t9242)
}
