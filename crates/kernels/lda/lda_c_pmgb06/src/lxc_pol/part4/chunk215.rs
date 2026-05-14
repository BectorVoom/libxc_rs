//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 215/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk215<F: Float>(t223: F, t607: F, t213: F, t224: F, t434: F, t438: F, t448: F, t462: F, t481: F, t488: F, t492: F, t502: F, t515: F, t533: F, t574: F, t583: F, t590: F, t593: F, t604: F) -> (F, F) {
    let t609 = 2.0 / 45.0 * t223 * t607;
    let t610 = t434 + t438 + t448 + t462 - t481 + t488 + t492 + t502 + t515 - t533 + t574 * t213 / 3.0 + t583 + t590 + t593 - t604 * t224 / 15.0 - t609;
    (t609, t610)
}
