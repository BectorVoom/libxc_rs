//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 222/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk222<F: Float>(t151: F, t161: F, t164: F, t709: F, t713: F, t756: F, t790: F, t806: F, t812: F, t827: F, t833: F, t843: F, t846: F, t851: F, t864: F, t870: F, t872: F, t874: F, t98: F) -> (F,) {
    let t879 = -2.427516195194328 * t790 * t98 + 1.8805371096875316 * t806 * t98 - t812 - 0.04115066352984959 * t164 * t827 - t833 - 1.8805371096875316 * t151 * t713 - 1.8805371096875316 * t151 * t709 + t843 + 0.04115066352984959 * t164 * t846 + 0.04115066352984959 * t164 * t851 - 4.937333717448355 * t161 * t709 + 4.937333717448355 * t864 * t98 - 4.937333717448355 * t161 * t713 - t870 + t872 - 0.04115066352984959 * t164 * t874 + 4.937333717448355 * t161 * t756;
    (t879,)
}
