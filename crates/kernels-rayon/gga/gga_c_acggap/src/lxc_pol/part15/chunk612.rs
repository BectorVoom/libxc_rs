//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 612/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk612(t1532: f64, t5799: f64, t1181: f64, t1761: f64, t3382: f64, t1165: f64, t1759: f64, t4298: f64, t1854: f64, t3201: f64, t398: f64, t1487: f64, t513: f64) -> (f64, f64, f64, f64, f64) {
    let t5800 = t1532 * t5799;
    let t5801 = t1181 * t5800;
    let t5804 = t3382 * t1761;
    let t5807 = t1165 * t4298 * t1759;
    let t5811 = t398 * t3201 * t1854;
    let t5814 = t1487 * t513;
    (t5801, t5804, t5807, t5811, t5814)
}
