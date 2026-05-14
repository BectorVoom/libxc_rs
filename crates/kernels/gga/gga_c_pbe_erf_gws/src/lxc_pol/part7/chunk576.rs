//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 576/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk576<F: Float>(t4442: F, t847: F, t2231: F, t338: F, t892: F, t2366: F, t2387: F, t833: F, t2362: F, t2384: F, t2388: F, t2397: F, t4385: F, t4390: F, t4397: F, t4402: F, t4405: F, t4410: F, t4415: F, t4419: F, t4425: F, t4427: F, t4430: F, t4433: F, t4438: F, t827: F, t844: F) -> (F, F, F) {
    let t4443 = t4442 * t847;
    let t4446 = t338 * t892 * t2231;
    let t4453 = t2387 * t2366;
    let t4454 = t4453 * t833;
    let t4456 = t4385 * t4390 / 16.0 - t4397 * t2362 / 32.0 - t827 * t4402 / 32.0 - t4405 * t2362 / 48.0 - t4410 * t2362 / 48.0 - 7.0 / 24.0 * t4415 + t827 * t4419 / 32.0 + 35.0 / 144.0 * t4425 + t4427 * t833 / 48.0 - 35.0 / 144.0 * t4430 - t844 * t4433 / 16.0 - t844 * t4438 / 48.0 - 35.0 / 72.0 * t4443 - t844 * t4446 / 16.0 + t2384 * t2397 / 32.0 + t2388 * t2397 / 32.0 - 7.0 / 96.0 * t4454;
    (t4446, t4453, t4456)
}
