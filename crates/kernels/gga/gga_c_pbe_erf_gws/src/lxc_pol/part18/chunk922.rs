//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 922/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk922<F: Float>(t11387: F, t2306: F, t3074: F, t1162: F, t3200: F, t338: F, t3717: F, t938: F, t2376: F, t2409: F, t1161: F, t2494: F, t11356: F, t11360: F, t11365: F, t11368: F, t11375: F, t11378: F, t11384: F, t2408: F, t3047: F, t3066: F, t3079: F, t335: F, t3733: F, t8654: F, t8695: F, t8771: F, t8776: F, t8780: F, t8793: F, t8803: F, t8810: F, t9241: F) -> (F, F, F, F, F) {
    let t11388 = t2306 * t11387;
    let t11389 = t3074 * t11388;
    let t11393 = t338 * t3200 * t1162;
    let t11396 = t3717 * t938;
    let t11398 = t2409 * t2376 * t11396;
    let t11401 = t2494 * t1161;
    let t11403 = t2409 * t2376 * t11401;
    let t11406 = t3066 * t11356 / 48.0 + t2408 * t11360 / 24.0 - t9241 * t11365 / 4.0 + 7.0 / 288.0 * t11368 - t8771 - t8654 * t3047 / 48.0 - t8776 * t3733 / 96.0 + t11375 * t11378 / 48.0 + t8793 * t8695 / 24.0 - t335 * t11384 / 48.0 + t11389 * t3079 / 96.0 + t8780 - t8803 - t8810 - t335 * t11393 / 48.0 + t2408 * t11398 / 48.0 + t2408 * t11403 / 24.0;
    (t11396, t11398, t11401, t11403, t11406)
}
