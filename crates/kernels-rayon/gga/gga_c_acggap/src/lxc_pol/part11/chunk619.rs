//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 619/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk619(t1501: f64, t336: f64, t839: f64, t1579: f64, t372: f64, t1143: f64, t1298: f64, t337: f64, t4099: f64, t1137: f64, t1503: f64, t3565: f64, t495: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4769 = t336 * t1501 * t839;
    let t4773 = t336 * t1579 * t372;
    let t4777 = t336 * t1143 * t1298;
    let t4781 = t336 * t337 * t4099;
    let t4785 = 7.0_f64 / 72.0_f64 * t1137 * t1503;
    let t4787 = t336 * t3565 * t495;
    (t4769, t4773, t4777, t4781, t4785, t4787)
}
