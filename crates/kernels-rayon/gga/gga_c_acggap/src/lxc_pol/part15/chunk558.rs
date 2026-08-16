//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 558/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk558(t1549: f64, t3382: f64, t1554: f64, t1558: f64, t1165: f64, t1539: f64, t4289: f64, t1163: f64, t1490: f64, t330: f64, t3740: f64, t527: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4320 = 0.85748036236139473944e-3_f64 * t3382 * t1549;
    let t4322 = 0.85748036236139473944e-3_f64 * t3382 * t1554;
    let t4324 = 0.42874018118069736972e-3_f64 * t3382 * t1558;
    let t4326 = t1165 * t4289 * t1539;
    let t4328 = 0.42874018118069736972e-3_f64 * t1163 * t4326;
    let t4339 = 7.0_f64 / 144.0_f64 * t330 * t1490;
    let t4340 = t3740 * t527;
    (t4320, t4322, t4324, t4326, t4328, t4339, t4340)
}
