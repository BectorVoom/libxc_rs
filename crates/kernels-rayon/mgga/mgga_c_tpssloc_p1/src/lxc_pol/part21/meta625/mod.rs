//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta625 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2405;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2406;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta625(t39264: f64, t761: f64, t2663: f64, t9901: f64, t2531: f64, t9905: f64, t39259: f64, t2250: f64, t2517: f64, t707: f64, t39358: f64, t756: f64, t187: f64, t268: f64, t39322: f64, t39347: f64, t39336: f64, t2652: f64, t9874: f64, t2244: f64, t2658: f64, t39488: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40679, t40680, t40682, t40685, t40687, t40708) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2405(t39264, t761, t2663, t9901, t2531, t9905, t39259, t2250, t2517, t707, t39358, t756);
        let (t40714, t40716, t40721, t40722, t40729, t40732) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2406(t187, t268, t39322, t39347, t39336, t761, t2652, t9874, t2244, t2517, t2658, t39488);
    (t40679, t40680, t40682, t40685, t40687, t40708, t40714, t40716, t40721, t40722, t40729, t40732)
}
