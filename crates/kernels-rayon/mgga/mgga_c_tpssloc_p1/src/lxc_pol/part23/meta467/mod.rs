//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta467 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1368;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1369;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1370;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1371;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1372;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1373;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta467(t2904: f64, t77139: f64, t951: f64, t959: f64, t21091: f64, t4483: f64, t17564: f64, t60722: f64, t21589: f64, t77119: f64, t77122: f64, t77124: f64, t77127: f64, t77130: f64, t77133: f64, t77135: f64, t77138: f64, t17934: f64, t5808: f64, t10523: f64, t76637: f64, t21095: f64, t48103: f64, t68442: f64, t68444: f64, t68446: f64, t68448: f64, t68452: f64, t68454: f64, t68494: f64, t68498: f64, t68500: f64, t77028: f64, t77030: f64, t77032: f64, t77034: f64, t59657: f64, t60168: f64, t60173: f64, t60204: f64, t68502: f64, t68504: f64, t68506: f64, t76877: f64, t76880: f64, t76887: f64, t76890: f64, t77042: f64, t77073: f64, t77076: f64, t41959: f64, t59688: f64, t59694: f64, t76574: f64, t76578: f64, t76583: f64, t76591: f64, t76599: f64, t76614: f64, t76622: f64, t76893: f64, t76896: f64, t76909: f64, t76915: f64, t41962: f64, t47787: f64, t76587: f64, t76595: f64, t76610: f64, t76618: f64, t76626: f64, t76899: f64, t76903: f64, t76906: f64, t76912: f64, t77102: f64, t77105: f64, t77107: f64, t942: f64, t13520: f64, t21253: f64, t10661: f64, t76644: f64, t913: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t77143, t77145, t77148, t77150, t77151) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1368(t2904, t77139, t951, t959, t21091, t4483, t17564, t60722, t21589, t77119, t77122, t77124, t77127, t77130, t77133, t77135, t77138);
        let (t77153, t77157, t77159, t77174) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1369(t17934, t5808, t10523, t76637, t951, t959, t21095, t4483, t48103, t68442, t68444, t68446, t68448, t68452, t68454, t68494, t68498, t68500, t77028, t77030, t77032, t77034);
        let t77189 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1370(t59657, t60168, t60173, t60204, t68502, t68504, t68506, t76877, t76880, t76887, t76890, t77042, t77073, t77076);
        let t77204 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1371(t41959, t59688, t59694, t76574, t76578, t76583, t76591, t76599, t76614, t76622, t76893, t76896, t76909, t76915);
        let t77218 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1372(t41962, t47787, t76587, t76595, t76610, t76618, t76626, t76899, t76903, t76906, t76912, t77102, t77105, t77107);
        let (t77220, t77224, t77226, t77229) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1373(t77174, t77189, t77204, t77218, t942, t951, t959, t13520, t21253, t10661, t76644, t913);
    (t77143, t77145, t77148, t77150, t77151, t77153, t77157, t77159, t77220, t77224, t77226, t77229)
}
