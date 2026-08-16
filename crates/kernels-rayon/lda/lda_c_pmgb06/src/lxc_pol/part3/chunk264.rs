//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 264/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk264(t153: f64, t813: f64, t137: f64, t132: f64, t473: f64, t809: f64, t103: f64, t466: f64, t471: f64, t811: f64) -> (f64, f64, f64, f64, f64) {
    let t814 = t813 * t153;
    let t815 = t137 * t814;
    let t817 = t132 * t815 / 30.0_f64;
    let t819 = t473 * t809;
    let t822 = -t466 - 0.035991666666666665_f64 * t811 - t471 - 0.006666666666666667_f64 * t103 * t819;
    (t814, t815, t817, t819, t822)
}
