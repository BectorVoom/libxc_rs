//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 971/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk971<F: Float>(t1652: F, t8800: F, t6376: F, t645: F, t797: F, t6403: F, t649: F, t36107: F, t6412: F, t8764: F, t6449: F, t7599: F) -> (F, F, F, F, F, F, F, F, F) {
    let t46072 = t8800 * t1652;
    let t46075 = t645 * t6376;
    let t46076 = t797 * t46075;
    let t46083 = t649 * t6403;
    let t46084 = t36107 * t46083;
    let t46086 = t649 * t6412;
    let t46087 = t8764 * t46086;
    let t46089 = t649 * t6449;
    let t46090 = t7599 * t46089;
    (t46072, t46075, t46076, t46083, t46084, t46086, t46087, t46089, t46090)
}
