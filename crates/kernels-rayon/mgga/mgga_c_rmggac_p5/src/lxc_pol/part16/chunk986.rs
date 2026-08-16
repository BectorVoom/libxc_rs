//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 986/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk986(t45167: f64, t7835: f64, t262: f64, t46237: f64, t35810: f64, t352: f64, t9884: f64, t35815: f64, t46228: f64, t7829: f64, t570: f64, t8700: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t46509 = t7835 * t45167;
    let t46511 = t262 * t46237;
    let t46512 = t35810 * t46511;
    let t46515 = t262 * t9884 * t352;
    let t46516 = t35815 * t46515;
    let t46522 = t262 * t46228;
    let t46523 = t7829 * t46522;
    let t46525 = t8700 * t570;
    (t46509, t46511, t46512, t46515, t46516, t46522, t46523, t46525)
}
