//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1212/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1212<F: Float>(t2408: F, t3060: F, t55151: F, t55781: F, t57441: F, t57449: F, t57454: F, t57458: F, t57462: F, t57468: F, t57472: F, t57474: F, t57476: F, t57480: F, t57482: F, t57484: F, t57486: F, t9283: F) -> (F,) {
    let t58818 = -t57441 / 768.0 + t57449 / 48.0 - t2408 * t9283 * t55151 * t3060 / 12.0 - t57454 / 768.0 + t57458 / 48.0 + t57462 / 1536.0 + t57468 / 48.0 - 7.0 / 144.0 * t57472 - t55781 - t57474 / 24.0 - 7.0 / 1152.0 * t57476 - t57480 / 48.0 - t57482 / 24.0 - t57484 / 12.0 - t57486 / 12.0;
    (t58818,)
}
