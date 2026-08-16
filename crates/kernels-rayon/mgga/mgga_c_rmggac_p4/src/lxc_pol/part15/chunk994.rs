//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 994/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk994(t1614: f64, t2402: f64, t874: f64, t9926: f64, t352: f64, t10189: f64, t333: f64, t2347: f64, t5267: f64, t25820: f64, t5888: f64, t27101: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t46375 = t2402 * t1614;
    let t46378 = t874 * t9926;
    let t46379 = t46378 * t352;
    let t46382 = t10189 * t333;
    let t46385 = t2347 * t5267;
    let t46386 = t25820 * t46385;
    let t46388 = t2347 * t5888;
    let t46389 = t27101 * t46388;
    (t46375, t46379, t46382, t46385, t46386, t46388, t46389)
}
