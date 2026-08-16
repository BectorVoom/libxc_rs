//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 922/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk922(t11798: f64, t1179: f64, t4068: f64, t871: f64, t2029: f64, t4119: f64, t2007: f64, t3213: f64, t131: f64, t1767: f64, t129: f64, t2012: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11799 = 0.09973633333333333_f64 * t11798;
    let t11810 = t871 * t1179 * t4068;
    let t11813 = t2029 * t4119;
    let t11860 = t3213 * t2007;
    let t11861 = 2.0_f64 / 135.0_f64 * t11860;
    let t11862 = t131 * t1767;
    let t11864 = t129 * t11862 * t2012;
    (t11799, t11810, t11813, t11861, t11862, t11864)
}
