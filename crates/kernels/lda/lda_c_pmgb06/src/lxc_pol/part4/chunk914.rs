//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 914/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk914<F: Float>(t1436: F, t1592: F, t1533: F, t947: F, t139: F, t1767: F, t134: F, t138: F, t1537: F, t1527: F, t3259: F, t1437: F, t1830: F, t455: F, t1530: F, t1490: F, t1554: F, t161: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9084 = t1436 * t1592;
    let t9147 = t947 * t1533;
    let t9175 = t1767 * t139;
    let t9177 = t138 * t9175 * t134;
    let t9178 = 0.01959135802469136 * t9177;
    let t9179 = t947 * t1537;
    let t9181 = t947 * t1527;
    let t9188 = t139 * t3259;
    let t9189 = t1437 * t1437;
    let t9190 = 1.0 / t9189;
    let t9215 = t1830 * t455;
    let t9220 = 1.0 / t1437 / t1530;
    let t9242 = t161 * t1554 * t1490;
    (t9084, t9147, t9175, t9177, t9178, t9179, t9181, t9188, t9190, t9215, t9220, t9242)
}
