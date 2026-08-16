//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 819/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk819(t9148: f64, t9375: f64, t9401: f64, t9445: f64, t105: f64, t469: f64, t8040: f64, t9089: f64, t3952: f64, t642: f64, t9098: f64, t1713: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9447 = t9148 + t9375 + t9401 + t9445;
    let t9448 = t105 * t9447;
    let t9449 = t9448 * t469;
    let t9455 = t8040 * t9089;
    let t9460 = t642 * t3952;
    let t9461 = t9460 * t9098;
    let t9469 = t469 * t1713;
    (t9447, t9448, t9449, t9455, t9460, t9461, t9469)
}
