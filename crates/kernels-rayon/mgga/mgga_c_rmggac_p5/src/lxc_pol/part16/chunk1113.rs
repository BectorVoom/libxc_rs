//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1113/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1113(t37536: f64, t41327: f64, t41340: f64, t41342: f64, t43598: f64, t43606: f64, t43611: f64, t46212: f64, t46214: f64, t46216: f64, t46218: f64, t46220: f64, t46222: f64, t46224: f64, t46226: f64, t46229: f64) -> f64 {
    let t49110 = t43598 + 0.67737888500486877234e-2_f64 * t41327 - t43606 - 0.21241846568096930143e-1_f64 * t41340 + 0.6386262240994031234e0_f64 * t41342 + t37536 + 0.14546486215597515589e0_f64 * t46212 + 0.67737888500486877232e-2_f64 * t46214 + 0.13637330827122670865e-1_f64 * t46216 - 0.2727466165424534173e-1_f64 * t46218 - t43611 - 0.21241846568096930142e-1_f64 * t46220 - 0.31931311204970156171e0_f64 * t46222 - 0.53104616420242325356e-2_f64 * t46224 + 0.79656924630363488034e-2_f64 * t46226 - 0.11151969448250888325e-1_f64 * t46229;
    t49110
}
