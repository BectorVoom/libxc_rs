//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 992/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk992(t46228: f64, t5162: f64, t1743: f64, t2064: f64, t797: f64, t46427: f64, t5148: f64, t9908: f64, t46501: f64, t5259: f64, t40826: f64, t9704: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t46677 = t5162 * t46228;
    let t46685 = t2064 * t1743;
    let t46686 = t797 * t46685;
    let t46702 = t5148 * t46427;
    let t46707 = t9908 * t2064;
    let t46710 = t5259 * t46501;
    let t46715 = t40826 * t9704;
    (t46677, t46685, t46686, t46702, t46707, t46710, t46715)
}
