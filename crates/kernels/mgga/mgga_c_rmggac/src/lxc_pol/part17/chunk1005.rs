//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1005/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1005<F: Float>(t262: F, t46228: F, t7829: F, t570: F, t8700: F, t7782: F, t1652: F, t2350: F, t10166: F, t321: F, t35824: F, t333: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
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
