//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 971/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk971(t1652: f64, t8800: f64, t6376: f64, t645: f64, t797: f64, t6403: f64, t649: f64, t36107: f64, t6412: f64, t8764: f64, t6449: f64, t7599: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
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
