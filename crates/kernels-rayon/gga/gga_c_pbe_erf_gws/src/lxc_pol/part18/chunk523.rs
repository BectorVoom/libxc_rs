//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 523/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk523(t1027: f64, t617: f64, t1809: f64, t1620: f64, t572: f64, t995: f64, t418: f64, t1821: f64, t1820: f64, t1000: f64, t562: f64, t1037: f64, t1627: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2575 = t1027 * t617;
    let t2576 = t1809 * t2575;
    let t2578 = 8.0_f64 / 45.0_f64 * t1620 * t2576;
    let t2579 = t995 * t572;
    let t2580 = t2579 * t418;
    let t2581 = t1821 * t2580;
    let t2583 = 8.0_f64 / 45.0_f64 * t1820 * t2581;
    let t2584 = t1000 * t562;
    let t2585 = t1821 * t2584;
    let t2587 = 8.0_f64 / 45.0_f64 * t1820 * t2585;
    let t2590 = 4.0_f64 / 45.0_f64 * t1627 * t1037;
    (t2575, t2576, t2578, t2579, t2580, t2581, t2583, t2584, t2585, t2587, t2590)
}
