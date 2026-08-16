//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 830/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk830(t39: f64, t55: f64, t59: f64, t87: f64, t2693: f64, t721: f64, t754: f64, t2965: f64, t807: f64, t286: f64, t688: f64, t796: f64) -> (f64, f64, f64, f64) {
    let t11549 = 24.0_f64 * t39 * t55 * t59 * t87;
    let t11552 = 0.71233333333333333332e-1_f64 * t721 * t754 * t2693;
    let t11553 = t2965 * t807;
    let t11557 = 0.21053605041484726346e2_f64 * t286 * t688 * t796;
    (t11549, t11552, t11553, t11557)
}
