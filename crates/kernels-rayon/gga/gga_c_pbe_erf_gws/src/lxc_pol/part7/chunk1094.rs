//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1094/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1094(t19561: f64, t2105: f64, t825: f64, t2367: f64, t6135: f64, t6084: f64, t829: f64, t830: f64, t831: f64, t2387: f64, t4384: f64, t2359: f64, t2362: f64, t2388: f64, t2392: f64, t2397: f64, t4396: f64, t4405: f64, t4410: f64, t4419: f64, t4427: f64, t4464: f64, t4484: f64, t6107: f64, t6111: f64, t6772: f64, t6778: f64, t6784: f64, t6800: f64, t6801: f64, t833: f64, t8782: f64) -> (f64, f64, f64) {
    let t19562 = t19561 * t2105;
    let t19563 = t19562 * t825;
    let t19581 = t2367 * t6135;
    let t19585 = t829 * t830 * t831 * t6084;
    let t19592 = t2387 * t4384;
    let t19595 = t8782 * t6801 * t6772 / 16.0_f64 - t6800 * t19563 * t2362 / 16.0_f64 + t6800 * t4396 * t6778 / 16.0_f64 + t6107 * t2397 / 24.0_f64 + t2387 * t6111 * t833 / 32.0_f64 + t2388 * t4419 / 16.0_f64 + t4427 * t2397 / 12.0_f64 - t4410 * t4464 / 32.0_f64 + 7.0_f64 / 6.0_f64 * t19581 - t2359 * t19585 / 96.0_f64 - t2392 * t6784 / 8.0_f64 - t4405 * t4464 / 32.0_f64 + t19592 * t4484 / 12.0_f64;
    (t19562, t19592, t19595)
}
