//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta681 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2491;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2492;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta681(t41115: f64, t4250: f64, t4166: f64, t9637: f64, t2649: f64, t13257: f64, t2617: f64, t4184: f64, t4257: f64, t9993: f64, t13176: f64, t2638: f64, t831: f64, t13278: f64, t2681: f64, t4236: f64, t9674: f64, t13186: f64, t2697: f64, t13289: f64, t41011: f64, t4179: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46649, t46657, t46658, t46661, t46663, t46667) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2491(t41115, t4250, t4166, t9637, t2649, t13257, t2617, t4184, t4257, t9993, t13176, t2638);
        let (t46668, t46675, t46677, t46679, t46686, t46692) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2492(t46667, t831, t13278, t2681, t4236, t9674, t13186, t2697, t13289, t41011, t4179, t820);
    (t46649, t46657, t46658, t46661, t46663, t46667, t46668, t46675, t46677, t46679, t46686, t46692)
}
