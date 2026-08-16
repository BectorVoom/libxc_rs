//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2259/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2259(t104450: f64, t105709: f64, t118: f64, t13537: f64, t29432: f64, t4293: f64, t7586: f64, t98455: f64, t98458: f64, t98461: f64, t98463: f64, t98467: f64, t98472: f64, t98474: f64, t98477: f64, t98483: f64, t98486: f64, t98489: f64, t98491: f64, t98494: f64, t98499: f64, t98501: f64, t98522: f64) -> f64 {
    let t105712 = -t98455 - t98458 + t98461 - t98463 - t98467 - t98472 - t98474 - t98477 - t98483 - t98486 - t98489 - t98491 - t98494 - t98499 + t98501 - 4.0_f64 * t29432 * t4293 - 2.0_f64 * t7586 * t13537 - t118 * (t104450 + t105709) - t98522;
    t105712
}
