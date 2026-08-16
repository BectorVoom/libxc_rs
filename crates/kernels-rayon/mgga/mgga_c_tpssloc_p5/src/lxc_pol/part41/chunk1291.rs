//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1291/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1291(t29895: f64, t30288: f64, t30294: f64, t29900: f64, t30298: f64, t2349: f64, t50: f64, t110143: f64, t8269: f64, t110532: f64, t30311: f64, t2281: f64, t8266: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t111104 = 4.0_f64 / 3.0_f64 * t29895 * t30288;
    let t111109 = 20.0_f64 / 9.0_f64 * t29895 * t30294;
    let t111111 = 50.0_f64 / 27.0_f64 * t29900 * t30298;
    let t111121 = t50 * t2349;
    let t111125 = t110143 * t8269;
    let t111127 = t110532 * t30311;
    let t111129 = t2281 * t8266;
    (t111104, t111109, t111111, t111121, t111125, t111127, t111129)
}
