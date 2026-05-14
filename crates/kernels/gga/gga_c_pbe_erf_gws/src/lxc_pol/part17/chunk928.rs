//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 928/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk928<F: Float>(t1161: F, t2417: F, t2409: F, t9296: F, t3076: F, t8945: F, t1118: F, t2220: F, t338: F, t3200: F, t845: F, t3097: F, t892: F, t1115: F, t2384: F, t2397: F, t2408: F, t2503: F, t3040: F, t3066: F, t3079: F, t3207: F, t4419: F, t6746: F, t6748: F, t6805: F, t844: F, t9275: F, t9285: F, t9289: F, t9290: F, t9293: F) -> (F, F, F) {
    let t9297 = t1161 * t2417;
    let t9299 = t2409 * t9296 * t9297;
    let t9302 = t8945 * t3076;
    let t9307 = t338 * t2220 * t1118;
    let t9313 = t338 * t3200 * t845;
    let t9317 = t338 * t892 * t3097;
    let t9320 = 35.0 / 432.0 * t9275 + t3040 * t2397 / 48.0 - 7.0 / 288.0 * t6746 + t2384 * t2503 / 96.0 - 7.0 / 144.0 * t6748 - t2408 * t9285 / 12.0 + t9289 - 35.0 / 216.0 * t9290 - t3207 * t9293 / 8.0 - t3066 * t9299 / 16.0 + t9302 * t3079 / 96.0 + 7.0 / 72.0 * t6805 - t844 * t9307 / 48.0 + t1115 * t4419 / 96.0 - t844 * t9313 / 24.0 - t844 * t9317 / 24.0;
    (t9297, t9299, t9320)
}
