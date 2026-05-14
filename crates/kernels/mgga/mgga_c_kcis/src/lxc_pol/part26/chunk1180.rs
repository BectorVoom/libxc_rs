//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1180/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1180<F: Float>(t102280: F, t102626: F, t102629: F, t102632: F, t102636: F, t102640: F, t102642: F, t27583: F, t95137: F, t99591: F, t99593: F, t99600: F, t99610: F, t1014: F, t29380: F, t2002: F, t303: F, t98607: F) -> (F, F, F) {
    let t102646 = -0.15476481481481481481e-2 * t102626 + 0.46429444444444444444e-2 * t102629 - t99591 - t99593 + t99600 + 0.17411041666666666666e-2 * t102632 - 0.61905925925925925924e-2 * t102636 + 0.7722800925925925926e-4 * t95137 - 0.34822083333333333332e-2 * t102640 - 0.41270617283950617283e-2 * t102642 - 0.46336805555555555556e-3 * t27583 * t102280 - t99610;
    let t102649 = t1014 * t29380;
    let t102653 = t303 * t98607 * t2002;
    (t102646, t102649, t102653)
}
