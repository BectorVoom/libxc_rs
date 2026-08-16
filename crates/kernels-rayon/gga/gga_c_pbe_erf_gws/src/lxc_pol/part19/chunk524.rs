//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 524/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk524(t331: f64, t641: f64, t34: f64, t643: f64, t639: f64, t1044: f64, t649: f64, t617: f64, t1621: f64, t1620: f64, t1791: f64, t661: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2591 = t331 * t641;
    let t2592 = t643 * t34;
    let t2593 = t2591 * t2592;
    let t2595 = 8.0_f64 / 45.0_f64 * t639 * t2593;
    let t2596 = t649 * t1044;
    let t2597 = t2596 * t617;
    let t2598 = t1621 * t2597;
    let t2600 = 4.0_f64 / 15.0_f64 * t1620 * t2598;
    let t2601 = t1791 * t1044;
    let t2602 = t2601 * t661;
    (t2591, t2592, t2593, t2595, t2597, t2598, t2600, t2601, t2602)
}
