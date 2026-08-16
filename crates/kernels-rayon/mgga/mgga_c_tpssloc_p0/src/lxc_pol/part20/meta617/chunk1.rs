//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2228/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2228(t12945: f64, t2427: f64, t12935: f64, t193: f64, t2522: f64, t39400: f64, t39408: f64, t39411: f64, t40708: f64, t40714: f64, t40716: f64, t4119: f64, t46207: f64, t46209: f64, t46213: f64, t776: f64) -> (f64, f64) {
    let t46217 = t2427 * t12945;
    let t46218 = 12.0_f64 * t46217;
    let t46219 = 18.0_f64 * t12935 * t193 * t4119 + 9.0_f64 * t2522 * t46213 * t776 - t39400 + t39408 + t39411 + t40708 - t40714 + t40716 + t46207 - t46209 + t46218;
    (t46218, t46219)
}
