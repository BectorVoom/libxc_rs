//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 554/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk554(t2416: f64, t2417: f64, t353: f64, t338: f64, t2222: f64, t2225: f64, t2228: f64, t2233: f64, t2236: f64, t2239: f64, t2244: f64, t2247: f64, t2355: f64, t2359: f64, t2362: f64, t2368: f64, t2373: f64, t2379: f64, t2384: f64, t2388: f64, t2392: f64, t2397: f64, t2401: f64, t2404: f64, t2408: f64, t2412: f64, t335: f64, t827: f64, t833: f64, t844: f64) -> (f64, f64, f64) {
    let t2418 = t2416 * t2417;
    let t2419 = t353 * t2418;
    let t2420 = t338 * t2419;
    let t2423 = t335 * t2222 / 96.0_f64 - 7.0_f64 / 144.0_f64 * t2225 - t335 * t2228 / 48.0_f64 - t844 * t2233 / 48.0_f64 + 7.0_f64 / 144.0_f64 * t2236 - t844 * t2239 / 24.0_f64 + t2244 + 7.0_f64 / 72.0_f64 * t2247 - t335 * t2355 / 96.0_f64 - t2359 * t2362 / 96.0_f64 - 7.0_f64 / 144.0_f64 * t2368 - t827 * t2373 / 24.0_f64 - t827 * t2379 / 48.0_f64 + t2384 * t833 / 96.0_f64 + t2388 * t833 / 96.0_f64 + t2392 * t833 / 96.0_f64 + t827 * t2397 / 48.0_f64 + t2401 * t2404 / 16.0_f64 + t2408 * t2412 / 24.0_f64 + t335 * t2420 / 48.0_f64;
    (t2418, t2420, t2423)
}
