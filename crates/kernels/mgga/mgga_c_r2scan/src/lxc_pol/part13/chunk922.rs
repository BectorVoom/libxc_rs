//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 922/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk922<F: Float>(t11665: F, t11689: F, t11723: F, t11755: F, t11779: F, t11814: F, t11833: F, t11852: F, t797: F, t1048: F, t499: F, t10991: F, t10996: F, t11008: F, t11018: F, t11549: F, t11553: F, t11558: F, t11562: F, t11616: F, t11619: F, t11624: F, t11628: F, t11632: F, t11635: F, t11638: F) -> (F, F, F, F) {
    let t11855 = t11665 + t11689 + t11723 + t11755 + t11779 + t11814 + t11833 + t11852;
    let t11856 = t11855 * t797;
    let t11858 = t1048 * t499 * t11856;
    let t11859 = t11858 / 4.0;
    let t11860 = t11549 - t11553 - t11558 - 0.40650199722100037752e-3 * t11616 + t10991 + t10996 - t11562 + t11619 - 0.40650199722100037752e-3 * t11008 - t11624 - t11018 - t11628 + t11632 + t11635 - t11638 - t11859;
    (t11855, t11856, t11858, t11860)
}
