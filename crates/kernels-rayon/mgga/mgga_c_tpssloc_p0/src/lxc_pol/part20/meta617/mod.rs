//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta617 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2227;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2228;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta617(t46171: f64, t46190: f64, t157: f64, t182: f64, t40687: f64, t4199: f64, t9905: f64, t2517: f64, t3966: f64, t707: f64, t9494: f64, t13471: f64, t870: f64, t12945: f64, t2427: f64, t12935: f64, t193: f64, t2522: f64, t39400: f64, t39408: f64, t39411: f64, t40708: f64, t40714: f64, t40716: f64, t4119: f64, t776: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46191, t46194, t46195, t46197, t46207, t46209, t46213) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2227(t46171, t46190, t157, t182, t40687, t4199, t9905, t2517, t3966, t707, t9494, t13471, t870);
        let (t46218, t46219) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2228(t12945, t2427, t12935, t193, t2522, t39400, t39408, t39411, t40708, t40714, t40716, t4119, t46207, t46209, t46213, t776);
    (t46191, t46194, t46195, t46197, t46207, t46209, t46218, t46219)
}
