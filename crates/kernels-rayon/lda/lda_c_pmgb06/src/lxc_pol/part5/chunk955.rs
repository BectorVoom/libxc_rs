//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 955/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk955(t247: f64, t4344: f64, t927: f64, t101: f64, t7245: f64, t754: f64, t757: f64, t328: f64, t113: f64, t301: f64, t395: f64, t6716: f64) -> (f64, f64, f64, f64) {
    let t14761 = t247 * t927 * t4344;
    let t14773 = t101 * t7245 * t754 * t757;
    let t14776 = t7245 * t328;
    let t14786 = t395 * t6716 * t113 * t301;
    (t14761, t14773, t14776, t14786)
}
