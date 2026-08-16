//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1019/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1019(t1161: f64, t2494: f64, t2376: f64, t2409: f64, t11356: f64, t11360: f64, t11365: f64, t11368: f64, t11375: f64, t11378: f64, t11384: f64, t11389: f64, t11393: f64, t11398: f64, t2408: f64, t3047: f64, t3066: f64, t3079: f64, t335: f64, t3733: f64, t8654: f64, t8695: f64, t8771: f64, t8776: f64, t8780: f64, t8793: f64, t8803: f64, t8810: f64, t9241: f64) -> (f64, f64, f64) {
    let t11401 = t2494 * t1161;
    let t11403 = t2409 * t2376 * t11401;
    let t11406 = t3066 * t11356 / 48.0_f64 + t2408 * t11360 / 24.0_f64 - t9241 * t11365 / 4.0_f64 + 7.0_f64 / 288.0_f64 * t11368 - t8771 - t8654 * t3047 / 48.0_f64 - t8776 * t3733 / 96.0_f64 + t11375 * t11378 / 48.0_f64 + t8793 * t8695 / 24.0_f64 - t335 * t11384 / 48.0_f64 + t11389 * t3079 / 96.0_f64 + t8780 - t8803 - t8810 - t335 * t11393 / 48.0_f64 + t2408 * t11398 / 48.0_f64 + t2408 * t11403 / 24.0_f64;
    (t11401, t11403, t11406)
}
