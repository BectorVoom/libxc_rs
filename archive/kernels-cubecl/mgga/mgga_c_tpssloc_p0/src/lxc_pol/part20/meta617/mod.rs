//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta617 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2227;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2228;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta617<F: Float>(t46171: F, t46190: F, t157: F, t182: F, t40687: F, t4199: F, t9905: F, t2517: F, t3966: F, t707: F, t9494: F, t13471: F, t870: F, t12945: F, t2427: F, t12935: F, t193: F, t2522: F, t39400: F, t39408: F, t39411: F, t40708: F, t40714: F, t40716: F, t4119: F, t776: F) -> (F, F, F, F, F, F, F, F) {
        let (t46191, t46194, t46195, t46197, t46207, t46209, t46213) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2227::<F>(t46171, t46190, t157, t182, t40687, t4199, t9905, t2517, t3966, t707, t9494, t13471, t870);
        let (t46218, t46219) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2228::<F>(t12945, t2427, t12935, t193, t2522, t39400, t39408, t39411, t40708, t40714, t40716, t4119, t46207, t46209, t46213, t776);
    (t46191, t46194, t46195, t46197, t46207, t46209, t46218, t46219)
}
