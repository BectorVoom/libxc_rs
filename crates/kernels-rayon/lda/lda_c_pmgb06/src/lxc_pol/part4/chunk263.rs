//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 263/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk263(t132: f64, t815: f64, t473: f64, t809: f64, t103: f64, t466: f64, t471: f64, t811: f64) -> (f64, f64, f64) {
    let t817 = t132 * t815 / 30.0_f64;
    let t819 = t473 * t809;
    let t822 = -t466 - 0.035991666666666665_f64 * t811 - t471 - 0.006666666666666667_f64 * t103 * t819;
    (t817, t819, t822)
}
