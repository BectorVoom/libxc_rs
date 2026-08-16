//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1362/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1362(t14979: f64, t15429: f64, t22534: f64, t2376: f64, t2408: f64, t2409: f64, t3066: f64, t3207: f64, t36129: f64, t3703: f64, t4110: f64, t4216: f64, t55137: f64, t55161: f64, t56434: f64, t56439: f64, t56442: f64, t56460: f64, t56474: f64, t56476: f64, t56483: f64, t56495: f64, t56500: f64, t8589: f64, t8629: f64) -> f64 {
    let t58172 = -5.0_f64 / 192.0_f64 * t56434 + t56439 / 768.0_f64 + t56442 / 192.0_f64 - t56460 / 384.0_f64 - t55161 - t3066 * t2409 * t22534 * t15429 / 16.0_f64 + 5.0_f64 / 192.0_f64 * t56474 + t3066 * t2409 * t36129 * t4216 / 24.0_f64 - t3207 * t2409 * t2376 * t4110 * t3703 / 16.0_f64 + t2408 * t2409 * t8589 * t14979 / 24.0_f64 - 7.0_f64 / 36.0_f64 * t56476 - t8629 * t55137 / 24.0_f64 - t56483 / 24.0_f64 - t56495 / 48.0_f64 + t56500 / 96.0_f64;
    t58172
}
