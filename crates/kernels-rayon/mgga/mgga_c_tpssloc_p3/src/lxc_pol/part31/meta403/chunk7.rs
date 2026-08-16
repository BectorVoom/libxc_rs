//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1485/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1485(t5456: f64, t649: f64, t5465: f64, t626: f64, t5464: f64, t9365: f64, t666: f64, t4043: f64, t4067: f64, t5489: f64, t2331: f64, t5488: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19461 = t649 * t5456;
    let t19471 = t626 * t5465;
    let t19473 = t9365 * t5464;
    let t19474 = t19473 * t666;
    let t19477 = t4043 * t4067;
    let t19480 = t626 * t5489;
    let t19482 = t2331 * t5488;
    (t19461, t19471, t19474, t19477, t19480, t19482)
}
