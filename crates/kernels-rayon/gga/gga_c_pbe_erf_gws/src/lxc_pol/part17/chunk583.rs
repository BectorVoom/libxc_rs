//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 583/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk583(t1000: f64, t562: f64, t1821: f64, t1820: f64, t1037: f64, t1627: f64, t331: f64, t641: f64, t34: f64, t643: f64, t639: f64, t1044: f64, t649: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2584 = t1000 * t562;
    let t2585 = t1821 * t2584;
    let t2587 = 8.0_f64 / 45.0_f64 * t1820 * t2585;
    let t2590 = 4.0_f64 / 45.0_f64 * t1627 * t1037;
    let t2591 = t331 * t641;
    let t2592 = t643 * t34;
    let t2593 = t2591 * t2592;
    let t2595 = 8.0_f64 / 45.0_f64 * t639 * t2593;
    let t2596 = t649 * t1044;
    (t2584, t2585, t2587, t2590, t2591, t2592, t2593, t2595, t2596)
}
