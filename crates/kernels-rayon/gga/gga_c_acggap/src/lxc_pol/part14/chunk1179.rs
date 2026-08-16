//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1179/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1179(t1165: f64, t30698: f64, t38789: f64, t604: f64, t5712: f64, t7561: f64, t5717: f64, t5722: f64, t1894: f64, t2095: f64, t355: f64, t2001: f64, t6116: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40251 = t30698 * t1165 * t604 * t38789;
    let t40253 = t7561 * t5712;
    let t40255 = t7561 * t5717;
    let t40257 = t7561 * t5722;
    let t40260 = t2095 * t1894 * t355;
    let t40262 = t2001 * t6116;
    (t40251, t40253, t40255, t40257, t40260, t40262)
}
