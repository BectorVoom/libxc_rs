//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 621/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk621(t345: f64, t4801: f64, t1480: f64, t3111: f64, t1298: f64, t355: f64, t721: f64, t1060: f64, t1072: f64, t495: f64, t3126: f64, t3124: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4802 = t345 * t4801;
    let t4804 = t3111 * t1480;
    let t4806 = t355 * t1298;
    let t4807 = t4806 * t721;
    let t4808 = t1060 * t4807;
    let t4809 = 0.12225e0_f64 * t4808;
    let t4810 = t1072 * t495;
    let t4811 = t4810 * t3126;
    let t4812 = t3124 * t4811;
    (t4802, t4804, t4806, t4808, t4809, t4810, t4812)
}
