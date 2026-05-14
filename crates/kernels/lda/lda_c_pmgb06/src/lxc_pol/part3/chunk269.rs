//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 269/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk269<F: Float>(t176: F, t842: F, t166: F, t161: F, t525: F, t838: F, t103: F, t519: F, t523: F, t840: F) -> (F, F, F, F, F) {
    let t843 = t842 * t176;
    let t844 = t166 * t843;
    let t846 = t161 * t844 / 30.0;
    let t848 = t525 * t838;
    let t851 = -t519 - 0.035991666666666665 * t840 - t523 - 0.006666666666666667 * t103 * t848;
    (t843, t844, t846, t848, t851)
}
