//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 769/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk769<F: Float>(t2255: F, t6298: F, t2189: F, t274: F, t343: F, t851: F, t6: F, t3235: F, t875: F, t2253: F, t2343: F, t3247: F, t6246: F, t6251: F, t6255: F, t6260: F, t6262: F, t6266: F, t6273: F, t6275: F, t6279: F, t6284: F, t6289: F, t6293: F, t902: F) -> (F, F, F, F, F, F) {
    let t6299 = t2255 * t6298;
    let t6303 = t274 * t2189 * t343;
    let t6304 = t851 * t6303;
    let t6305 = t2255 * t6304;
    let t6308 = t6 * t2189;
    let t6310 = t3235 * t6308 * t875;
    let t6313 = -t6246 + t6251 - t6255 - t6260 + t902 * t6262 / F::cast_from(768.0_f64) + t902 * t6266 / F::cast_from(1536.0_f64) + t6273 + t6275 * t6279 / F::cast_from(32.0_f64) + t2343 * t6284 / F::cast_from(128.0_f64) - F::cast_from(3.0_f64) / F::cast_from(128.0_f64) * t3247 * t6289 - t2253 * t6293 / F::cast_from(256.0_f64) - t2253 * t6299 / F::cast_from(256.0_f64) - t2253 * t6305 / F::cast_from(256.0_f64) - t2343 * t6310 / F::cast_from(512.0_f64);
    (t6299, t6303, t6305, t6308, t6310, t6313)
}
