//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 983/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk983(t30526: f64, t9708: f64, t25525: f64, t321: f64, t9884: f64, t333: f64, t25529: f64, t3826: f64, t45730: f64, t25518: f64, t45568: f64, t25636: f64, t45572: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t46232 = t30526 * t9708;
    let t46235 = t25525 * t9884 * t321;
    let t46237 = t9884 * t333;
    let t46238 = t25529 * t46237;
    let t46242 = t3826 * t45730;
    let t46244 = t25518 * t45568;
    let t46246 = t25636 * t45572;
    (t46232, t46235, t46237, t46238, t46242, t46244, t46246)
}
