//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta740 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2437;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2438;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2439;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2440;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2441;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2442;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta740(t13520: f64, t17507: f64, t13727: f64, t17510: f64, t10661: f64, t4395: f64, t5730: f64, t21303: f64, t42028: f64, t912: f64, t21300: f64, t2792: f64, t21299: f64, t2844: f64, t2842: f64, t10702: f64, t17527: f64, t21252: f64, t42100: f64, t42102: f64, t10811: f64, t14271: f64, t14276: f64, t17492: f64, t17544: f64, t17548: f64, t17551: f64, t21115: f64, t2930: f64, t4416: f64, t4438: f64, t4471: f64, t59920: f64, t60407: f64, t931: f64, t21194: f64, t2888: f64, t41684: f64, t48799: f64, t48800: f64, t48809: f64, t59657: f64, t68442: f64, t68444: f64, t68446: f64, t68448: f64, t68479: f64, t68483: f64, t68486: f64, t68489: f64, t68492: f64, t68494: f64, t68498: f64, t68571: f64, t68577: f64, t68580: f64, t68583: f64, t42245: f64, t47787: f64, t59663: f64, t59665: f64, t59680: f64, t59688: f64, t59694: f64, t59700: f64, t59702: f64, t59704: f64, t59759: f64, t59761: f64, t68586: f64, t68589: f64, t68592: f64, t68596: f64, t68599: f64, t68602: f64, t68605: f64, t68608: f64, t10771: f64, t14442: f64, t17366: f64, t17554: f64, t17555: f64, t21114: f64, t21195: f64, t21198: f64, t21207: f64, t21239: f64, t21242: f64, t2861: f64, t2886: f64, t2900: f64, t311: f64, t41821: f64, t42128: f64, t42154: f64, t42226: f64, t42228: f64, t4433: f64, t4449: f64, t49285: f64, t49411: f64, t5758: f64, t5762: f64, t5794: f64, t68702: f64, t943: f64, t951: f64, t300: f64, t69050: f64, t69180: f64, t69218: f64, t69249: f64, t69286: f64, t69326: f64, t14459: f64, t17947: f64, t959: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t69335, t69337, t69340, t69343, t69346) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2437(t13520, t17507, t13727, t17510, t10661, t4395, t5730, t21303, t42028, t912, t21300, t2792);
        let (t69350, t69353, t69357, t69368) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2438(t21299, t2844, t2842, t912, t10702, t17527, t4395, t21252, t42100, t42102, t10811, t14271, t14276, t17492, t17544, t17548, t17551, t21115, t2930, t4416, t4438, t4471, t59920, t60407, t69335, t69337, t69340, t69343, t69346, t931);
        let (t69380, t69425) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2439(t21194, t2888, t41684, t48799, t48800, t48809, t59657, t68442, t68444, t68446, t68448, t68479, t68483, t68486, t68489, t68492, t68494, t68498, t68571, t68577, t68580, t68583);
        let t69445 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2440(t42245, t47787, t59663, t59665, t59680, t59688, t59694, t59700, t59702, t59704, t59759, t59761, t68586, t68589, t68592, t68596, t68599, t68602, t68605, t68608);
        let t69449 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2441(t10771, t10811, t14442, t17366, t17554, t17555, t21114, t21195, t21198, t21207, t21239, t21242, t2861, t2886, t2900, t311, t41821, t42128, t42154, t42226, t42228, t4433, t4449, t49285, t49411, t5758, t5762, t5794, t68702, t69380, t69425, t69445, t931, t943, t951);
        let (t69453, t69456) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2442(t300, t69050, t69180, t69218, t69249, t69286, t69326, t69368, t69449, t14459, t17947, t959);
    (t69335, t69337, t69340, t69343, t69346, t69350, t69353, t69357, t69453, t69456)
}
