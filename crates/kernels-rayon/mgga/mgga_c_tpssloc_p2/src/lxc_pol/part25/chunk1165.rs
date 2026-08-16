//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1165/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1165(t23258: f64, t6547: f64, t794: f64, t852: f64, t6562: f64, t6572: f64, t6552: f64, t6555: f64, t82124: f64, t23035: f64, t23237: f64, t23241: f64) -> (f64, f64, f64, f64, f64) {
    let t82131 = t6547 * t23258;
    let t82133 = t794 * t852;
    let t82135 = t6562 * t82133 * t6572;
    let t82138 = t6552 * t82124 * t6555;
    let t82141 = t23035 * t23237 * t23241;
    (t82131, t82133, t82135, t82138, t82141)
}
