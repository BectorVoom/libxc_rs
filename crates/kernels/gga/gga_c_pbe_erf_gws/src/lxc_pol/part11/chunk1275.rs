//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1275/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1275<F: Float>(t2168: F, t4386: F, t50354: F, t1076: F, t1123: F, t1133: F, t21640: F, t21647: F, t2253: F, t2255: F, t2343: F, t3373: F, t343: F, t3854: F, t50349: F, t50353: F, t50362: F, t50363: F, t50368: F, t9665: F) -> (F, F) {
    let t50371 = t2168 * t4386 * t50354 / F::new(4.0);
    let t50372 = -t2253 * t2255 * t1123 * t3373 * t1133 * t343 / F::new(192.0) - t2253 * t2255 * t1123 * t1076 * t3854 * t343 / F::new(128.0) + t21640 + t21647 + t50349 + t50353 + t2343 * t9665 * t50354 / F::new(32.0) + t50362 - t50363 + t50368 + t50371;
    (t50371, t50372)
}
