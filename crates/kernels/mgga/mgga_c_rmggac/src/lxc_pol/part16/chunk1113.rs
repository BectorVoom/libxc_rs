//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1113/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1113<F: Float>(t37536: F, t41327: F, t41340: F, t41342: F, t43598: F, t43606: F, t43611: F, t46212: F, t46214: F, t46216: F, t46218: F, t46220: F, t46222: F, t46224: F, t46226: F, t46229: F) -> F {
    let t49110 = t43598 + F::new(0.67737888500486877234e-2) * t41327 - t43606 - F::new(0.21241846568096930143e-1) * t41340 + F::new(0.6386262240994031234e0) * t41342 + t37536 + F::new(0.14546486215597515589e0) * t46212 + F::new(0.67737888500486877232e-2) * t46214 + F::new(0.13637330827122670865e-1) * t46216 - F::new(0.2727466165424534173e-1) * t46218 - t43611 - F::new(0.21241846568096930142e-1) * t46220 - F::new(0.31931311204970156171e0) * t46222 - F::new(0.53104616420242325356e-2) * t46224 + F::new(0.79656924630363488034e-2) * t46226 - F::new(0.11151969448250888325e-1) * t46229;
    t49110
}
