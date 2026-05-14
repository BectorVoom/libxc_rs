//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 939/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk939<F: Float>(t9619: F, t12648: F, t12650: F, t12653: F, t12654: F, t12655: F, t12656: F, t12657: F, t12659: F, t12662: F, t12664: F, t12665: F, t132: F, t137: F, t822: F, t9590: F) -> (F, F, F) {
    let t12666 = 2.0 / 45.0 * t9619;
    let t12667 = -t12648 - t12650 - t12653 - t12654 + t12655 + t12656 - 8.0 / 405.0 * t12657 - 2.0 / 15.0 * t12659 + t12662 - t12664 - t12665 - t12666;
    let t12672 = t132 * t137 * t9590 * t822 / 30.0;
    (t12666, t12667, t12672)
}
