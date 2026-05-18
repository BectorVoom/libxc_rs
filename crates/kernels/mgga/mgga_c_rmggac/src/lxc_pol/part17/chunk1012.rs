//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1012/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1012<F: Float>(t2073: F, t46122: F, t2079: F, t262: F, t36: F, t6463: F, t27041: F, t45568: F, t27091: F, t45572: F, t25877: F, t45577: F) -> (F, F, F, F, F) {
    let t46652 = t2073 * t46122;
    let t46656 = t2079 * t262 * t36 * t6463;
    let t46658 = t27041 * t45568;
    let t46660 = t27091 * t45572;
    let t46662 = t25877 * t45577;
    (t46652, t46656, t46658, t46660, t46662)
}
