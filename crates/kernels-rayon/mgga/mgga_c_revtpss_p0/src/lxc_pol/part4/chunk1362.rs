//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1362/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1362(t1042: f64, t17203: f64, t3172: f64, t5298: f64, t3711: f64, t1469: f64, t3568: f64, t5296: f64, t5278: f64, t1250: f64, t17170: f64, t482: f64) -> (f64, f64, f64, f64, f64) {
    let t17204 = t1042 * t17203;
    let t17209 = t3172 * t5298;
    let t17211 = 0.19055119163586549765e-3_f64 * t3711 * t17209;
    let t17212 = t1469 * t3568;
    let t17213 = t5296 * t17212;
    let t17214 = t1042 * t17213;
    let t17217 = t3172 * t5278;
    let t17219 = 0.19055119163586549765e-3_f64 * t3711 * t17217;
    let t17221 = t482 * t17170 * t1250;
    (t17204, t17211, t17214, t17219, t17221)
}
