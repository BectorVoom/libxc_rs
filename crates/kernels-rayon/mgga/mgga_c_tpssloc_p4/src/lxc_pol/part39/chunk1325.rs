//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1325/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1325(t30349: f64, t580: f64, t1404: f64, t8283: f64, t1858: f64, t8199: f64, t110280: f64, t110282: f64, t110484: f64, t110919: f64, t111243: f64, t111284: f64, t1396: f64, t1398: f64, t16507: f64, t16546: f64, t2206: f64, t2212: f64, t30095: f64, t30350: f64, t30395: f64) -> f64 {
    let t111289 = 2.0_f64 * t30349 * t580;
    let t111291 = 2.0_f64 * t8283 * t1404;
    let t111293 = 2.0_f64 * t8199 * t1858;
    let t111296 = t16507 * t2212 + t30095 * t1858 + 2.0_f64 * t30350 * t1404 + 2.0_f64 * t1396 * t30395 + t110919 + t1398 * (t111243 + t111284) + 2.0_f64 * t110484 + t111289 + t111291 + t110280 + t111293 + t2206 * t16546 + 2.0_f64 * t110282;
    t111296
}
