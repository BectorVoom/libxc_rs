//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1394/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1394(t2408: f64, t3060: f64, t55151: f64, t55781: f64, t57441: f64, t57449: f64, t57454: f64, t57458: f64, t57462: f64, t57468: f64, t57472: f64, t57474: f64, t57476: f64, t57480: f64, t57482: f64, t57484: f64, t57486: f64, t9283: f64) -> f64 {
    let t58818 = -t57441 / 768.0_f64 + t57449 / 48.0_f64 - t2408 * t9283 * t55151 * t3060 / 12.0_f64 - t57454 / 768.0_f64 + t57458 / 48.0_f64 + t57462 / 1536.0_f64 + t57468 / 48.0_f64 - 7.0_f64 / 144.0_f64 * t57472 - t55781 - t57474 / 24.0_f64 - 7.0_f64 / 1152.0_f64 * t57476 - t57480 / 48.0_f64 - t57482 / 24.0_f64 - t57484 / 12.0_f64 - t57486 / 12.0_f64;
    t58818
}
