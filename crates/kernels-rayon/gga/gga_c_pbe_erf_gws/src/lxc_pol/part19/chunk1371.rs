//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1371/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1371(t15431: f64, t9270: f64, t15482: f64, t2376: f64, t2408: f64, t2409: f64, t2494: f64, t26654: f64, t3066: f64, t3067: f64, t3068: f64, t3721: f64, t4110: f64, t4207: f64, t4227: f64, t53750: f64, t54598: f64, t54599: f64, t55351: f64, t56776: f64, t56783: f64, t56787: f64, t56791: f64, t56793: f64, t56799: f64, t56811: f64, t56813: f64, t9296: f64, t938: f64) -> f64 {
    let t58431 = t9270 * t15431;
    let t58444 = t56776 / 12.0_f64 - t55351 - t56783 / 24.0_f64 + t56787 / 768.0_f64 - t56791 / 192.0_f64 - t56793 / 48.0_f64 + t54598 * t54599 * t4207 * t3068 / 4.0_f64 + t56799 / 24.0_f64 + t3066 * t2409 * t3067 * t15482 * t938 / 48.0_f64 - t3066 * t2409 * t9296 * t4110 * t3721 / 16.0_f64 + 7.0_f64 / 48.0_f64 * t58431 + t2408 * t2409 * t26654 * t4207 / 24.0_f64 + t2408 * t2409 * t2376 * t4227 * t2494 / 24.0_f64 - t53750 - t56811 / 384.0_f64 + t56813 / 12.0_f64;
    t58444
}
