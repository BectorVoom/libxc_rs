//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1007/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1007<F: Float>(t814: F, t9488: F, t20264: F, t20272: F, t20278: F, t20280: F, t20284: F, t20285: F, t20291: F, t20301: F, t20304: F, t20306: F, t20308: F, t20312: F, t2253: F, t2255: F, t2278: F, t2312: F, t254: F, t3223: F, t6579: F, t851: F) -> (F,) {
    let t20314 = t9488 * t814;
    let t20319 = -t2253 * t254 * t20264 * t3223 / 192.0 - 119.0 / 288.0 * t20272 - t20278 - t20280 + t20284 + t2312 * t2255 * t2278 * t20285 / 96.0 - t2253 * t2255 * t851 * t20291 / 192.0 - t20301 - t20304 * t20306 * t20308 / 16.0 - 7.0 / 48.0 * t20312 - 5.0 / 64.0 * t6579 * t2255 * t2278 * t20314;
    (t20319,)
}
