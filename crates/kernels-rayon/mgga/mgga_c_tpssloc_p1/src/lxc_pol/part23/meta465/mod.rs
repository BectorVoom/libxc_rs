//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta465 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1360;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1361;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1362;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1363;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta465(t17210: f64, t5705: f64, t21180: f64, t4362: f64, t17218: f64, t4378: f64, t48103: f64, t68442: f64, t68444: f64, t68446: f64, t68448: f64, t68452: f64, t68454: f64, t68494: f64, t68498: f64, t68500: f64, t2815: f64, t41904: f64, t47787: f64, t59657: f64, t76574: f64, t76578: f64, t76583: f64, t76587: f64, t76591: f64, t76595: f64, t76599: f64, t59688: f64, t59694: f64, t76610: f64, t76614: f64, t76618: f64, t76622: f64, t76626: f64, t901: f64, t5698: f64, t41935: f64, t60168: f64, t60173: f64, t60204: f64, t68502: f64, t68504: f64, t68506: f64, t76877: f64, t76880: f64, t76887: f64, t76890: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t77028, t77030, t77032, t77034, t77037) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1360(t17210, t5705, t21180, t4362, t17218, t4378, t48103, t68442, t68444, t68446, t68448, t68452, t68454, t68494, t68498, t68500);
        let (t77041, t77042, t77058) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1361(t5705, t2815, t41904, t47787, t59657, t68442, t76574, t76578, t76583, t76587, t76591, t76595, t76599);
        let t77071 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1362(t59688, t59694, t68444, t68446, t68448, t68494, t68498, t76610, t76614, t76618, t76622, t76626);
        let (t77072, t77073, t77075, t77076, t77082) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1363(t77058, t77071, t901, t5698, t41935, t59657, t60168, t60173, t60204, t68502, t68504, t68506, t76877, t76880, t76887, t76890, t77042);
    (t77028, t77030, t77032, t77034, t77037, t77041, t77042, t77072, t77073, t77075, t77076, t77082)
}
