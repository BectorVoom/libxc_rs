//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 536/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk536(t3670: f64, t425: f64, t431: f64, t438: f64, t3237: f64, t1005: f64, t993: f64, t174: f64, t3101: f64, t386: f64, t387: f64, t384: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3671 = t3670 * t425;
    let t3673 = t3670 * t431;
    let t3677 = t3670 * t438;
    let t3679 = t3237 * t425;
    let t3694 = 0.64311027177104605458e-3_f64 * t1005 * t993;
    let t3695 = t174 * t3101;
    let t3697 = t386 * t387 * t3695;
    let t3699 = 0.21437009059034868486e-3_f64 * t384 * t3697;
    (t3671, t3673, t3677, t3679, t3694, t3697, t3699)
}
