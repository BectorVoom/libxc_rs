//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1132/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1132<F: Float>(t6241: F, t3222: F, t6203: F, t6392: F, t814: F, t9488: F, t20264: F, t20272: F, t20278: F, t20280: F, t20284: F, t20285: F, t20291: F, t20301: F, t20304: F, t20306: F, t2253: F, t2255: F, t2278: F, t2312: F, t254: F, t3223: F, t6579: F, t851: F, param_a_c: F) -> F {
    let t20307 = t6241 * param_a_c;
    let t20308 = t20307 * t3222;
    let t20312 = t6203 * t6392;
    let t20314 = t9488 * t814;
    let t20319 = -t2253 * t254 * t20264 * t3223 / F::cast_from(192.0_f64) - F::cast_from(119.0_f64) / F::cast_from(288.0_f64) * t20272 - t20278 - t20280 + t20284 + t2312 * t2255 * t2278 * t20285 / F::cast_from(96.0_f64) - t2253 * t2255 * t851 * t20291 / F::cast_from(192.0_f64) - t20301 - t20304 * t20306 * t20308 / F::cast_from(16.0_f64) - F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t20312 - F::cast_from(5.0_f64) / F::cast_from(64.0_f64) * t6579 * t2255 * t2278 * t20314;
    t20319
}
