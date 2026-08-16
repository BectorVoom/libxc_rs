//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 554/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk554<F: Float>(t2416: F, t2417: F, t353: F, t338: F, t2222: F, t2225: F, t2228: F, t2233: F, t2236: F, t2239: F, t2244: F, t2247: F, t2355: F, t2359: F, t2362: F, t2368: F, t2373: F, t2379: F, t2384: F, t2388: F, t2392: F, t2397: F, t2401: F, t2404: F, t2408: F, t2412: F, t335: F, t827: F, t833: F, t844: F) -> (F, F, F) {
    let t2418 = t2416 * t2417;
    let t2419 = t353 * t2418;
    let t2420 = t338 * t2419;
    let t2423 = t335 * t2222 / F::cast_from(96.0_f64) - F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t2225 - t335 * t2228 / F::cast_from(48.0_f64) - t844 * t2233 / F::cast_from(48.0_f64) + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t2236 - t844 * t2239 / F::cast_from(24.0_f64) + t2244 + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t2247 - t335 * t2355 / F::cast_from(96.0_f64) - t2359 * t2362 / F::cast_from(96.0_f64) - F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t2368 - t827 * t2373 / F::cast_from(24.0_f64) - t827 * t2379 / F::cast_from(48.0_f64) + t2384 * t833 / F::cast_from(96.0_f64) + t2388 * t833 / F::cast_from(96.0_f64) + t2392 * t833 / F::cast_from(96.0_f64) + t827 * t2397 / F::cast_from(48.0_f64) + t2401 * t2404 / F::cast_from(16.0_f64) + t2408 * t2412 / F::cast_from(24.0_f64) + t335 * t2420 / F::cast_from(48.0_f64);
    (t2418, t2420, t2423)
}
