//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 578/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk578(t1630: f64, t3077: f64, t1629: f64, t955: f64, t150: f64, t2934: f64, t119: f64, t2937: f64, t943: f64, t945: f64, t1651: f64, t930: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4192 = t3077 * t1630;
    let t4194 = t1629 * t955;
    let t4197 = t150 * t2934;
    let t4198 = t119 * t4197;
    let t4199 = t2937 * t943;
    let t4200 = t1629 * t4199;
    let t4203 = t1629 * t945;
    let t4206 = t1651 * t930;
    (t4192, t4194, t4198, t4199, t4200, t4203, t4206)
}
