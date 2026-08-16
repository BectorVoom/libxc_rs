//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1005/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1005(t262: f64, t46228: f64, t7829: f64, t570: f64, t8700: f64, t7782: f64, t1652: f64, t2350: f64, t10166: f64, t321: f64, t35824: f64, t333: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t46522 = t262 * t46228;
    let t46523 = t7829 * t46522;
    let t46525 = t8700 * t570;
    let t46526 = t262 * t46525;
    let t46527 = t7782 * t46526;
    let t46529 = t2350 * t1652;
    let t46530 = t262 * t46529;
    let t46531 = t7782 * t46530;
    let t46533 = t10166 * t321;
    let t46534 = t262 * t46533;
    let t46535 = t35824 * t46534;
    let t46537 = t10166 * t333;
    (t46522, t46523, t46525, t46526, t46527, t46529, t46530, t46531, t46533, t46534, t46535, t46537)
}
