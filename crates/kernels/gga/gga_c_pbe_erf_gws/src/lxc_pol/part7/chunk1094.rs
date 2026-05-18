//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1094/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1094<F: Float>(t19561: F, t2105: F, t825: F, t2367: F, t6135: F, t6084: F, t829: F, t830: F, t831: F, t2387: F, t4384: F, t2359: F, t2362: F, t2388: F, t2392: F, t2397: F, t4396: F, t4405: F, t4410: F, t4419: F, t4427: F, t4464: F, t4484: F, t6107: F, t6111: F, t6772: F, t6778: F, t6784: F, t6800: F, t6801: F, t833: F, t8782: F) -> (F, F, F) {
    let t19562 = t19561 * t2105;
    let t19563 = t19562 * t825;
    let t19581 = t2367 * t6135;
    let t19585 = t829 * t830 * t831 * t6084;
    let t19592 = t2387 * t4384;
    let t19595 = t8782 * t6801 * t6772 / F::new(16.0) - t6800 * t19563 * t2362 / F::new(16.0) + t6800 * t4396 * t6778 / F::new(16.0) + t6107 * t2397 / F::new(24.0) + t2387 * t6111 * t833 / F::new(32.0) + t2388 * t4419 / F::new(16.0) + t4427 * t2397 / F::new(12.0) - t4410 * t4464 / F::new(32.0) + F::new(7.0) / F::new(6.0) * t19581 - t2359 * t19585 / F::new(96.0) - t2392 * t6784 / F::new(8.0) - t4405 * t4464 / F::new(32.0) + t19592 * t4484 / F::new(12.0);
    (t19562, t19592, t19595)
}
