//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2278/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2278<F: Float>(t39549: F, t40779: F, t40784: F, t40790: F, t40793: F, t40797: F, t40799: F, t40801: F, t40803: F, t46303: F, t46309: F, t46311: F, t46313: F, t46314: F, t46315: F, t46318: F, t46319: F) -> F {
    let t47145 = t46303 - t40779 + t40784 - t46309 - t46311 + t40790 + t40793 + t46313 + t40797 + t40799 + t40801 - t40803 - t46314 + t46315 + t46318 + t46319 + t39549;
    t47145
}
