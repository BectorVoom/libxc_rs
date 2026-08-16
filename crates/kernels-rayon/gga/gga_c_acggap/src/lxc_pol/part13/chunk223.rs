//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 223/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk223(t60: f64, t803: f64, t40: f64, t123: f64, t203: f64, t84: f64, t281: f64, t467: f64, t191: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t804 = t60 * t803;
    let t805 = t40 * t804;
    let t807 = t203 * t123 * t84;
    let t808 = t281 * t807;
    let t809 = 0.24415263074675393405e-3_f64 * t808;
    let t811 = t467 * t467;
    let t813 = t191 * t191;
    let t814 = 1.0_f64 / t813;
    (t804, t805, t807, t809, t811, t813, t814)
}
