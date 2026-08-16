//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 970/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk970(t1704: f64, t265: f64, t262: f64, t7648: f64, t1737: f64, t7653: f64, t2115: f64, t46129: f64, t2118: f64, t46177: f64, t7633: f64, t7641: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t46180 = t265 * t1704;
    let t46181 = t262 * t46180;
    let t46182 = t7648 * t46181;
    let t46184 = t265 * t1737;
    let t46185 = t262 * t46184;
    let t46186 = t7653 * t46185;
    let t46189 = t2115 * t46129;
    let t46191 = t2118 * t46177;
    let t46193 = t7633 * t46181;
    let t46195 = t7641 * t46185;
    (t46180, t46181, t46182, t46184, t46185, t46186, t46189, t46191, t46193, t46195)
}
