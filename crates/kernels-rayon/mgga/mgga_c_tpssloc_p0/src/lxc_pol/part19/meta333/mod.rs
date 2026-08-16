//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta333 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1194;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1195;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1196;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta333(t39358: f64, t756: f64, t706: f64, t9448: f64, t708: f64, t187: f64, t268: f64, t39322: f64, t39347: f64, t39336: f64, t761: f64, t2652: f64, t9874: f64, t2523: f64, t39400: f64, t39408: f64, t39411: f64, t39463: f64, t39468: f64, t39472: f64, t39476: f64, t4314: f64, t9616: f64, t751: f64, t9288: f64, t9897: f64, t2244: f64, t2517: f64, t2658: f64, t39488: f64, t2531: f64, t9919: f64, t707: f64, t9258: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40708, t40711, t40714, t40716, t40721, t40722) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1194(t39358, t756, t706, t9448, t708, t187, t268, t39322, t39347, t39336, t761, t2652, t9874);
        let (t40723, t40724) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1195(t40722, t2523, t39400, t39408, t39411, t39463, t39468, t39472, t39476, t40708, t40711, t40714, t40716, t40721, t4314, t9616);
        let (t40727, t40730, t40732, t40734, t40736) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1196(t751, t9288, t9897, t2244, t2517, t2658, t39488, t761, t2531, t9919, t707, t9258);
    (t40708, t40711, t40714, t40716, t40721, t40723, t40724, t40727, t40730, t40732, t40734, t40736)
}
