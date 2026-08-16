//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1034/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1034<F: Float>(t2310: F, t9087: F, t2412: F, t8597: F, t1982: F, t7428: F, t9775: F, t9735: F, t2186: F, t9790: F, t46764: F, t739: F) -> (F, F, F, F, F, F) {
    let t46992 = t9087 * t2310;
    let t46995 = t2412 * t8597;
    let t46999 = t9775 * t7428 * t1982;
    let t47004 = t9735 * t7428 * t1982;
    let t47006 = t2186 * t9790;
    let t47008 = t739 * t46764;
    (t46992, t46995, t46999, t47004, t47006, t47008)
}
