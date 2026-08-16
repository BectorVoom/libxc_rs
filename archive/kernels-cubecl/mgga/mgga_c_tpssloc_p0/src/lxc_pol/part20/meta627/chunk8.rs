//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2274/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2274<F: Float>(t39249: F, t39256: F, t39309: F, t39312: F, t39316: F, t39320: F, t40679: F, t46120: F, t46126: F, t46129: F, t46131: F, t46133: F, t46135: F, t46138: F, t46140: F, t46141: F, t46142: F) -> F {
    let t47138 = -t39249 + t46120 - t46126 - t39256 - t46129 - t46131 - t46133 + t46135 + t46138 - t39309 + t39312 + t39316 + t39320 - t46140 - t46141 + t46142 - t40679;
    t47138
}
