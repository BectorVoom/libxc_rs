//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 842/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk842(t145: f64, t4875: f64, t1101: f64, t360: f64, t1106: f64, t372: f64, t322: f64, t955: f64, t361: f64, t435: f64, t171: f64, t3300: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16314 = t4875 * t145;
    let t16325 = t1101 * t360;
    let t16507 = t1106 * t372;
    let t16548 = t955 * t322;
    let t17752 = t361 * t435;
    let t17912 = t171 * t3300;
    (t16314, t16325, t16507, t16548, t17752, t17912)
}
