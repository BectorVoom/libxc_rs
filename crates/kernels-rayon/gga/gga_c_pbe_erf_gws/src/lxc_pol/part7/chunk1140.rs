//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1140/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1140(t6183: f64, t6325: f64, t6327: f64, t346: f64, t6472: f64, t6800: f64, t2150: f64, t6702: f64, t6707: f64, t20400: f64, t20401: f64, t20403: f64, t20410: f64, t20414: f64, t20416: f64, t20424: f64, t20428: f64, t2253: f64, t2277: f64, t3257: f64, t6195: f64, t6609: f64, t9482: f64) -> (f64, f64, f64, f64, f64) {
    let t20430 = t6325 * t6183 * t6327;
    let t20431 = 7.0_f64 / 24.0_f64 * t20430;
    let t20432 = t6472 * t346;
    let t20433 = t6800 * t20432;
    let t20435 = t20433 * t2150 / 12.0_f64;
    let t20437 = t6702 * t6707 / 32.0_f64;
    let t20438 = -t20400 + 7.0_f64 / 96.0_f64 * t20401 - 7.0_f64 / 384.0_f64 * t2277 * t3257 * t6195 * t20403 + t20410 + t20414 - t2253 * t9482 * t6609 * t20416 / 48.0_f64 + t20424 + t20428 - t20431 - t20435 - t20437;
    (t20431, t20432, t20435, t20437, t20438)
}
