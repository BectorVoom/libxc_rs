//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1014/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1014<F: Float>(t46357: F, t5259: F, t40823: F, t9708: F, t45726: F, t5271: F, t46529: F, t4669: F, t46228: F, t5162: F, t1737: F, t2124: F) -> (F, F, F, F, F, F) {
    let t46669 = t5259 * t46357;
    let t46671 = t40823 * t9708;
    let t46673 = t5271 * t45726;
    let t46675 = t4669 * t46529;
    let t46677 = t5162 * t46228;
    let t46679 = t2124 * t1737;
    (t46669, t46671, t46673, t46675, t46677, t46679)
}
