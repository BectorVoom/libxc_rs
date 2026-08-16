//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 700/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk700(t1101: f64, t1165: f64, t604: f64, t7493: f64, t1106: f64, t1181: f64, t7426: f64, t2085: f64, t372: f64, t4262: f64, t2030: f64, t182: f64, t592: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7495 = t1165 * t604 * t1101;
    let t7496 = t7493 * t7495;
    let t7497 = 0.15724046144802076034e-2_f64 * t7496;
    let t7499 = t1181 * t604 * t1106;
    let t7500 = t7426 * t7499;
    let t7502 = t2085 * t372;
    let t7503 = t4262 * t7502;
    let t7504 = t2030 * t7503;
    let t7506 = t182 * t592;
    (t7495, t7496, t7497, t7499, t7500, t7502, t7503, t7504, t7506)
}
