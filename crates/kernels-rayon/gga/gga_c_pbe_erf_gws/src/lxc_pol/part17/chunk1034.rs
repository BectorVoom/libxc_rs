//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1034/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1034(t2255: f64, t9337: f64, t3228: f64, t6402: f64, t6365: f64, t904: f64, t8891: f64, t1123: f64, t6297: f64, t2253: f64, t2277: f64, t2343: f64, t6685: f64, t8821: f64, t8823: f64, t8826: f64, t8831: f64, t8832: f64, t8835: f64, t8839: f64, t9334: f64) -> (f64, f64, f64, f64, f64) {
    let t9338 = t2255 * t9337;
    let t9342 = 7.0_f64 / 576.0_f64 * t6402 * t3228;
    let t9343 = t6365 * t904;
    let t9344 = t9343 * t8891;
    let t9347 = t1123 * t6297;
    let t9348 = t2255 * t9347;
    let t9351 = t2277 * t9334 / 256.0_f64 + t6685 * t9338 / 256.0_f64 - t8821 + t8823 + t9342 + t8826 + t8831 - 5.0_f64 / 192.0_f64 * t2343 * t9344 + t8832 - t2253 * t9348 / 384.0_f64 + t8835 - t8839;
    (t9338, t9344, t9347, t9348, t9351)
}
