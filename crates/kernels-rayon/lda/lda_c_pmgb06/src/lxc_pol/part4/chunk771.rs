//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 771/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk771(t132: f64, t5115: f64, t1592: f64, t813: f64, t1594: f64, t137: f64, t1604: f64, t831: f64, t1392: f64, t802: f64, t1631: f64, t3051: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5117 = 2.0_f64 / 45.0_f64 * t132 * t5115;
    let t5118 = t813 * t1592;
    let t5119 = t5118 * t1594;
    let t5120 = t137 * t5119;
    let t5122 = t132 * t5120 / 15.0_f64;
    let t5124 = t831 * t1604 / 15.0_f64;
    let t5126 = 2.0_f64 / 45.0_f64 * t802 * t1392;
    let t5128 = t802 * t1631 / 30.0_f64;
    let t5129 = t3051 / 45.0_f64;
    (t5117, t5118, t5119, t5120, t5122, t5124, t5126, t5128, t5129)
}
