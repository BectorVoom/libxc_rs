//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta652 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2400;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2401;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2402;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2403;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2404;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2405;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2406;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2407;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta652(t41662: f64, t41675: f64, t41678: f64, t41682: f64, t41684: f64, t41863: f64, t41865: f64, t41870: f64, t41872: f64, t41874: f64, t41876: f64, t48982: f64, t47761: f64, t47765: f64, t47769: f64, t48112: f64, t48114: f64, t48116: f64, t48119: f64, t48122: f64, t48125: f64, t48128: f64, t48131: f64, t41887: f64, t41889: f64, t48134: f64, t48137: f64, t48142: f64, t48145: f64, t48148: f64, t49009: f64, t49012: f64, t49015: f64, t49018: f64, t49021: f64, t48155: f64, t41680: f64, t41713: f64, t47777: f64, t48153: f64, t48157: f64, t48159: f64, t48161: f64, t48163: f64, t48165: f64, t48167: f64, t49040: f64, t41959: f64, t41962: f64, t47781: f64, t47785: f64, t47787: f64, t49043: f64, t49049: f64, t49052: f64, t49054: f64, t49056: f64, t49058: f64, t49060: f64, t49127: f64, t49140: f64, t49154: f64, t1556: f64, t2842: f64, t10727: f64, t10702: f64, t10704: f64, t2836: f64, t912: f64, t10655: f64, t14422: f64, t2793: f64, t4396: f64, t10662: f64, t4399: f64, t10828: f64, t1580: f64, t10524: f64, t10724: f64, t10740: f64, t10743: f64, t10771: f64, t10811: f64, t10825: f64, t14329: f64, t14425: f64, t14429: f64, t14435: f64, t14463: f64, t1581: f64, t2861: f64, t2862: f64, t2880: f64, t4434: f64, t4437: f64, t931: f64, t943: f64, t951: f64, t13515: f64, t2837: f64, t2841: f64, t4351: f64, t2845: f64, t10697: f64, t4354: f64, t10701: f64, t1543: f64, t10705: f64, t1557: f64, t41618: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t49167 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2400(t41662, t41675, t41678, t41682, t41684, t41863, t41865, t41870, t41872, t41874, t41876, t48982);
        let (t49181, t49194) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2401(t47761, t47765, t47769, t48112, t48114, t48116, t48119, t48122, t48125, t48128, t48131, t41887, t41889, t48134, t48137, t48142, t48145, t48148, t49009, t49012, t49015, t49018, t49021);
        let t49208 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2402(t48155, t41680, t41713, t47777, t48153, t48157, t48159, t48161, t48163, t48165, t48167, t49040);
        let t49219 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2403(t41959, t41962, t47781, t47785, t47787, t49043, t49049, t49052, t49054, t49056, t49058, t49060);
        let (t49222, t49228, t49240) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2404(t49127, t49140, t49154, t49167, t49181, t49194, t49208, t49219, t1556, t2842, t10727, t10702);
        let (t49244, t49256, t49259, t49262) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2405(t10704, t2836, t49240, t912, t10655, t14422, t2793, t2842, t4396, t10662, t10702, t4399);
        let t49266 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2406(t10828, t1580, t10524, t10724, t10740, t10743, t10771, t10811, t10825, t14329, t14425, t14429, t14435, t14463, t1581, t2861, t2862, t2880, t4434, t4437, t49222, t49228, t49244, t49256, t49259, t49262, t931, t943, t951);
        let (t49268, t49271, t49273, t49276, t49278) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2407(t13515, t2837, t2841, t4351, t2845, t10697, t4354, t10701, t1543, t10705, t1557, t41618);
    (t49222, t49228, t49244, t49256, t49259, t49262, t49266, t49268, t49271, t49273, t49276, t49278)
}
