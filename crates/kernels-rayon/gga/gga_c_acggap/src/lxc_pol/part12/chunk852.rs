//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 852/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk852(t157: f64, t20432: f64, t435: f64, t507: f64, t495: f64, t930: f64, t1298: f64, t407: f64, t5746: f64, t943: f64, t955: f64, t1188: f64, t1410: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20433 = t20432 * t157;
    let t20559 = t507 * t435;
    let t20590 = t930 * t495;
    let t20595 = t407 * t1298;
    let t20775 = t5746 * t943;
    let t20817 = t955 * t495;
    let t20935 = t1188 * t1410;
    (t20433, t20559, t20590, t20595, t20775, t20817, t20935)
}
