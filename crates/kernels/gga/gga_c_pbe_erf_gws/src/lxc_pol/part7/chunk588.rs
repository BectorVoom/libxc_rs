//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 588/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk588<F: Float>(t2362: F, t2384: F, t2388: F, t2397: F, t4385: F, t4390: F, t4397: F, t4402: F, t4405: F, t4410: F, t4415: F, t4419: F, t4425: F, t4427: F, t4430: F, t4433: F, t4438: F, t4443: F, t4446: F, t4454: F, t827: F, t833: F, t844: F) -> F {
    let t4456 = t4385 * t4390 / F::cast_from(16.0_f64) - t4397 * t2362 / F::cast_from(32.0_f64) - t827 * t4402 / F::cast_from(32.0_f64) - t4405 * t2362 / F::cast_from(48.0_f64) - t4410 * t2362 / F::cast_from(48.0_f64) - F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t4415 + t827 * t4419 / F::cast_from(32.0_f64) + F::cast_from(35.0_f64) / F::cast_from(144.0_f64) * t4425 + t4427 * t833 / F::cast_from(48.0_f64) - F::cast_from(35.0_f64) / F::cast_from(144.0_f64) * t4430 - t844 * t4433 / F::cast_from(16.0_f64) - t844 * t4438 / F::cast_from(48.0_f64) - F::cast_from(35.0_f64) / F::cast_from(72.0_f64) * t4443 - t844 * t4446 / F::cast_from(16.0_f64) + t2384 * t2397 / F::cast_from(32.0_f64) + t2388 * t2397 / F::cast_from(32.0_f64) - F::cast_from(7.0_f64) / F::cast_from(96.0_f64) * t4454;
    t4456
}
