//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1132/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1132(t6241: f64, t3222: f64, t6203: f64, t6392: f64, t814: f64, t9488: f64, t20264: f64, t20272: f64, t20278: f64, t20280: f64, t20284: f64, t20285: f64, t20291: f64, t20301: f64, t20304: f64, t20306: f64, t2253: f64, t2255: f64, t2278: f64, t2312: f64, t254: f64, t3223: f64, t6579: f64, t851: f64, param_a_c: f64) -> f64 {
    let t20307 = t6241 * param_a_c;
    let t20308 = t20307 * t3222;
    let t20312 = t6203 * t6392;
    let t20314 = t9488 * t814;
    let t20319 = -t2253 * t254 * t20264 * t3223 / 192.0_f64 - 119.0_f64 / 288.0_f64 * t20272 - t20278 - t20280 + t20284 + t2312 * t2255 * t2278 * t20285 / 96.0_f64 - t2253 * t2255 * t851 * t20291 / 192.0_f64 - t20301 - t20304 * t20306 * t20308 / 16.0_f64 - 7.0_f64 / 48.0_f64 * t20312 - 5.0_f64 / 64.0_f64 * t6579 * t2255 * t2278 * t20314;
    t20319
}
