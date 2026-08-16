//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 184/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk184(t336: f64, t337: f64, t570: f64, t22: f64, t326: f64, t130: f64) -> (f64, f64, f64, f64) {
    let t571 = t336 * t337;
    let t572 = t570 * t571;
    let t575 = 1.0_f64 / t22 / t326;
    let t576 = t130 * t575;
    (t571, t572, t575, t576)
}
