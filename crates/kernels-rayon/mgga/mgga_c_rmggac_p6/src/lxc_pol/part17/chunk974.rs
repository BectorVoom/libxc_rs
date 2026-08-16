//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 974/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk974(t36: f64, t6376: f64, t262: f64, t2118: f64, t22: f64, t30526: f64, t9885: f64, t1734: f64, t265: f64, t2100: f64, t46117: f64, t2103: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t46121 = t36 * t6376;
    let t46122 = t262 * t46121;
    let t46123 = t2118 * t46122;
    let t46126 = t30526 * t22 * t9885;
    let t46128 = t265 * t1734;
    let t46129 = t262 * t46128;
    let t46130 = t2100 * t46129;
    let t46133 = t2100 * t46117;
    let t46135 = t2103 * t46122;
    (t46121, t46122, t46123, t46126, t46128, t46129, t46130, t46133, t46135)
}
