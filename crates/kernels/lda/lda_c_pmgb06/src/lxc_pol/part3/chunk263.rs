//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 263/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk263<F: Float>(t153: F, t813: F, t137: F, t132: F, t473: F, t809: F, t103: F, t466: F, t471: F, t811: F) -> (F, F, F, F, F) {
    let t814 = t813 * t153;
    let t815 = t137 * t814;
    let t817 = t132 * t815 / 30.0;
    let t819 = t473 * t809;
    let t822 = -t466 - 0.035991666666666665 * t811 - t471 - 0.006666666666666667 * t103 * t819;
    (t814, t815, t817, t819, t822)
}
