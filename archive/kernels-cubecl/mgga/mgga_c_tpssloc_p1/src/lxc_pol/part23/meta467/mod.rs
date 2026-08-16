//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta467 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1368;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1369;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1370;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1371;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1372;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1373;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta467<F: Float>(t2904: F, t77139: F, t951: F, t959: F, t21091: F, t4483: F, t17564: F, t60722: F, t21589: F, t77119: F, t77122: F, t77124: F, t77127: F, t77130: F, t77133: F, t77135: F, t77138: F, t17934: F, t5808: F, t10523: F, t76637: F, t21095: F, t48103: F, t68442: F, t68444: F, t68446: F, t68448: F, t68452: F, t68454: F, t68494: F, t68498: F, t68500: F, t77028: F, t77030: F, t77032: F, t77034: F, t59657: F, t60168: F, t60173: F, t60204: F, t68502: F, t68504: F, t68506: F, t76877: F, t76880: F, t76887: F, t76890: F, t77042: F, t77073: F, t77076: F, t41959: F, t59688: F, t59694: F, t76574: F, t76578: F, t76583: F, t76591: F, t76599: F, t76614: F, t76622: F, t76893: F, t76896: F, t76909: F, t76915: F, t41962: F, t47787: F, t76587: F, t76595: F, t76610: F, t76618: F, t76626: F, t76899: F, t76903: F, t76906: F, t76912: F, t77102: F, t77105: F, t77107: F, t942: F, t13520: F, t21253: F, t10661: F, t76644: F, t913: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t77143, t77145, t77148, t77150, t77151) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1368::<F>(t2904, t77139, t951, t959, t21091, t4483, t17564, t60722, t21589, t77119, t77122, t77124, t77127, t77130, t77133, t77135, t77138);
        let (t77153, t77157, t77159, t77174) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1369::<F>(t17934, t5808, t10523, t76637, t951, t959, t21095, t4483, t48103, t68442, t68444, t68446, t68448, t68452, t68454, t68494, t68498, t68500, t77028, t77030, t77032, t77034);
        let t77189 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1370::<F>(t59657, t60168, t60173, t60204, t68502, t68504, t68506, t76877, t76880, t76887, t76890, t77042, t77073, t77076);
        let t77204 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1371::<F>(t41959, t59688, t59694, t76574, t76578, t76583, t76591, t76599, t76614, t76622, t76893, t76896, t76909, t76915);
        let t77218 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1372::<F>(t41962, t47787, t76587, t76595, t76610, t76618, t76626, t76899, t76903, t76906, t76912, t77102, t77105, t77107);
        let (t77220, t77224, t77226, t77229) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1373::<F>(t77174, t77189, t77204, t77218, t942, t951, t959, t13520, t21253, t10661, t76644, t913);
    (t77143, t77145, t77148, t77150, t77151, t77153, t77157, t77159, t77220, t77224, t77226, t77229)
}
