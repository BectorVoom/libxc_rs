//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2276/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2276<F: Float>(t39463: F, t39468: F, t39472: F, t39476: F, t39483: F, t40721: F, t40732: F, t46209: F, t46218: F, t46228: F, t46232: F, t46235: F, t46237: F, t46238: F, t46239: F, t46245: F, t46256: F) -> F {
    let t47141 = -t46209 + t46218 + t39463 - t39468 - t40721 - t46228 - t39472 - t39476 + t46232 - t46235 + t46237 + t46238 - t40732 - t46239 + t46245 + t46256 + t39483;
    t47141
}
