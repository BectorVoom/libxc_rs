//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 938/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk938(t1630: f64, t5224: f64, t639: f64, t16973: f64, t5003: f64, t642: f64, t17456: f64, t17461: f64, t17463: f64, t17465: f64, t17467: f64, t17469: f64, t17473: f64, t17476: f64, t17481: f64) -> (f64, f64, f64) {
    let t17483 = t639 * t1630 * t5224;
    let t17484 = 64.0_f64 / 45.0_f64 * t17483;
    let t17488 = 32.0_f64 / 15.0_f64 * t639 * t642 * t5003 * t16973;
    let t17489 = -t17456 - t17461 + t17463 + t17465 + t17467 - t17469 + t17473 - t17476 - t17481 + t17484 - t17488;
    (t17484, t17488, t17489)
}
