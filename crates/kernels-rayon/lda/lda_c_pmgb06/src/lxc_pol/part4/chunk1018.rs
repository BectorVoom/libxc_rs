//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1018/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1018(t9177: f64, t1697: f64, t1730: f64, t2852: f64, t432: f64, t132: f64, t2851: f64, t459: f64, t1179: f64, t136: f64, t154: f64, t1554: f64, t1587: f64, t161: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9724 = 0.3732469135802469_f64 * t9177;
    let t9759 = 0.19947266666666666_f64 * t1697 * t1730;
    let t9762 = t432 * t2852;
    let t9765 = t132 * t2851 * t459;
    let t9770 = 28.0_f64 / 1215.0_f64 * t132 * t1179 * t136 * t154;
    let t9774 = t161 * t1554 * t1587;
    (t9724, t9759, t9762, t9765, t9770, t9774)
}
