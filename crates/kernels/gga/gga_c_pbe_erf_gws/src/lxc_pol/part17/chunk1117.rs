//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1117/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1117<F: Float>(t2408: F, t3060: F, t50881: F, t51084: F, t51572: F, t51595: F, t53700: F, t53704: F, t53713: F, t53715: F, t53721: F, t53726: F, t53728: F, t53730: F, t53734: F, t53736: F, t53742: F, t8629: F, t9283: F) -> (F,) {
    let t53744 = -t53700 / 96.0 - 35.0 / 216.0 * t51572 - t53704 - 7.0 / 72.0 * t51595 - t2408 * t9283 * t51084 * t3060 / 12.0 - t53713 / 512.0 + t53715 / 96.0 - t53721 / 1536.0 - t53726 + t53728 - t53730 + t53734 / 48.0 - t53736 / 48.0 + t8629 * t50881 / 96.0 + t53742 / 1536.0;
    (t53744,)
}
