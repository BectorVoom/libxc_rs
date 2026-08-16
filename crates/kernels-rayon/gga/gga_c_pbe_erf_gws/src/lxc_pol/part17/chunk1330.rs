//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1330/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1330(t14031: f64, t9382: f64, t9552: f64, t4028: f64, t9116: f64, t51459: f64, t51461: f64, t51466: f64, t51473: f64, t51479: f64, t54391: f64, t54394: f64, t54398: f64, t54402: f64, t54404: f64) -> f64 {
    let t54406 = t14031 * t9382;
    let t54408 = t14031 * t9552;
    let t54411 = t4028 * t9116;
    let t54413 = -t51459 - 7.0_f64 / 48.0_f64 * t51461 - t54391 / 4.0_f64 + 7.0_f64 / 288.0_f64 * t51466 - t54394 / 16.0_f64 + 7.0_f64 / 288.0_f64 * t51473 + t54398 - t54402 - t54404 / 96.0_f64 - t54406 / 384.0_f64 - t54408 / 384.0_f64 + 7.0_f64 / 1152.0_f64 * t51479 - t54411 / 96.0_f64;
    t54413
}
