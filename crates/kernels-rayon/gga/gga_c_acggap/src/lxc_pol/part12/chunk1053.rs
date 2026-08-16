//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1053/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1053(t1165: f64, t30327: f64, t4358: f64, t604: f64, t30861: f64, t8458: f64, t5265: f64, t7351: f64, t8463: f64, t1181: f64, t5106: f64, t2264: f64, t30792: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34722 = t30327 * t1165 * t604 * t4358;
    let t34724 = t30861 * t8458;
    let t34728 = t8463 * t1165 * t7351 * t5265;
    let t34732 = t8463 * t1181 * t604 * t5106;
    let t34736 = t8463 * t1181 * t604 * t5265;
    let t34738 = t30792 * t2264;
    (t34722, t34724, t34728, t34732, t34736, t34738)
}
