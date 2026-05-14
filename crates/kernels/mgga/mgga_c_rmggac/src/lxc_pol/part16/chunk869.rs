//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 869/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk869<F: Float>(t27041: F, t45568: F, t27091: F, t45572: F, t25877: F, t45577: F, t45730: F, t5271: F, t46357: F, t5259: F, t40823: F, t9708: F, t45726: F, t46529: F, t4669: F, t46228: F, t5162: F) -> (F, F, F, F, F, F, F, F, F) {
    let t46658 = t27041 * t45568;
    let t46660 = t27091 * t45572;
    let t46662 = t25877 * t45577;
    let t46664 = t5271 * t45730;
    let t46669 = t5259 * t46357;
    let t46671 = t40823 * t9708;
    let t46673 = t5271 * t45726;
    let t46675 = t4669 * t46529;
    let t46677 = t5162 * t46228;
    (t46658, t46660, t46662, t46664, t46669, t46671, t46673, t46675, t46677)
}
