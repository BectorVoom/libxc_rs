//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 808/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk808(t355: f64, t535: f64, t2095: f64, t5720: f64, t599: f64, t1181: f64, t7337: f64, t5606: f64, t604: f64, t2068: f64, t1165: f64, t7351: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8771 = t535 * t355;
    let t8772 = t2095 * t8771;
    let t8774 = t599 * t5720;
    let t8775 = t1181 * t8774;
    let t8776 = t7337 * t8775;
    let t8778 = t604 * t5606;
    let t8779 = t1181 * t8778;
    let t8780 = t2068 * t8779;
    let t8783 = t1165 * t604 * t5720;
    let t8784 = t7337 * t8783;
    let t8787 = t1165 * t7351 * t5606;
    (t8771, t8772, t8774, t8775, t8776, t8778, t8779, t8780, t8783, t8784, t8787)
}
