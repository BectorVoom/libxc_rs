//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 902/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk902(t2852: f64, t432: f64, t132: f64, t2851: f64, t459: f64, t1179: f64, t136: f64, t154: f64, t1499: f64, t1504: f64, t1554: f64, t1587: f64, t161: f64) -> (f64, f64, f64, f64, f64) {
    let t9762 = t432 * t2852;
    let t9765 = t132 * t2851 * t459;
    let t9770 = 28.0_f64 / 1215.0_f64 * t132 * t1179 * t136 * t154;
    let t9771 = t1499 * t1504;
    let t9774 = t161 * t1554 * t1587;
    (t9762, t9765, t9770, t9771, t9774)
}
