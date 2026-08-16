//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta653 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;
mod chunk10;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2408;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2409;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2410;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2411;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2412;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2413;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2414;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2415;
use chunk8::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2416;
use chunk9::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2417;
use chunk10::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2418;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta653(t10650: f64, t4396: f64, t13655: f64, t2787: f64, t10810: f64, t1561: f64, t47705: f64, t47681: f64, t47686: f64, t47691: f64, t47695: f64, t47699: f64, t47703: f64, t48085: f64, t48087: f64, t48090: f64, t48092: f64, t47707: f64, t48096: f64, t41831: f64, t41833: f64, t47709: f64, t47711: f64, t47713: f64, t47715: f64, t47717: f64, t47722: f64, t47724: f64, t47728: f64, t47730: f64, t41656: f64, t41658: f64, t41660: f64, t47732: f64, t47736: f64, t47738: f64, t47744: f64, t47748: f64, t48098: f64, t48101: f64, t48103: f64, t41662: f64, t41675: f64, t41678: f64, t41682: f64, t41684: f64, t41863: f64, t41865: f64, t41870: f64, t41872: f64, t41874: f64, t41876: f64, t48982: f64, t47761: f64, t47765: f64, t47769: f64, t48112: f64, t48114: f64, t48116: f64, t48119: f64, t48122: f64, t48125: f64, t48128: f64, t48131: f64, t41887: f64, t41889: f64, t48134: f64, t48137: f64, t48142: f64, t48145: f64, t48148: f64, t49009: f64, t49012: f64, t49015: f64, t49018: f64, t49021: f64, t48155: f64, t48157: f64, t41680: f64, t41713: f64, t47777: f64, t48153: f64, t48159: f64, t48161: f64, t48163: f64, t48165: f64, t48167: f64, t49040: f64, t42212: f64, t42213: f64, t47781: f64, t47785: f64, t47787: f64, t49043: f64, t49049: f64, t49052: f64, t49054: f64, t49056: f64, t49058: f64, t49060: f64, t14363: f64, t942: f64, t10760: f64, t10806: f64, t10814: f64, t14329: f64, t14332: f64, t1569: f64, t2856: f64, t2925: f64, t42117: f64, t4411: f64, t4434: f64, t49268: f64, t49271: f64, t49273: f64, t49276: f64, t49278: f64, t924: f64, t932: f64, t952: f64, t2929: f64, t4446: f64, t1568: f64, t2886: f64, t10737: f64, t13520: f64, t2860: f64, t4408: f64, t10770: f64, t10524: f64, t10720: f64, t10734: f64, t10743: f64, t10747: f64, t10753: f64, t10756: f64, t10765: f64, t10772: f64, t10805: f64, t14263: f64, t14439: f64, t14443: f64, t14450: f64, t2863: f64, t2906: f64, t2930: f64, t2933: f64, t42020: f64, t42149: f64, t42226: f64, t42228: f64, t4437: f64, t4449: f64, t4454: f64, t4472: f64, t4475: f64, t10811: f64, t14255: f64, t892: f64, t914: f64, t2791: f64, t4351: f64, t2794: f64, t10660: f64, t1543: f64, t10663: f64, t10603: f64, t10813: f64, t10825: f64, t10828: f64, t14344: f64, t14366: f64, t14370: f64, t14453: f64, t14456: f64, t14459: f64, t14460: f64, t1581: f64, t2862: f64, t2880: f64, t2905: f64, t2924: f64, t41816: f64, t41821: f64, t42128: f64, t4476: f64, t931: f64, t950: f64, t300: f64, t48786: f64, t48861: f64, t49076: f64, t49113: f64, t49266: f64, t41769: f64, t4496: f64, t959: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t49280, t49282, t49285, t49305) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2408(t10650, t4396, t13655, t2787, t10810, t1561, t47705, t47681, t47686, t47691, t47695, t47699, t47703, t48085, t48087, t48090, t48092);
        let t49318 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2409(t47707, t48096, t41831, t41833, t47709, t47711, t47713, t47715, t47717, t47722, t47724, t47728);
        let t49332 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2410(t47730, t41656, t41658, t41660, t47732, t47736, t47738, t47744, t47748, t48098, t48101, t48103);
        let t49345 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2411(t41662, t41675, t41678, t41682, t41684, t41863, t41865, t41870, t41872, t41874, t41876, t48982);
        let (t49359, t49372) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2412(t47761, t47765, t47769, t48112, t48114, t48116, t48119, t48122, t48125, t48128, t48131, t41887, t41889, t48134, t48137, t48142, t48145, t48148, t49009, t49012, t49015, t49018, t49021);
        let t49386 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2413(t48155, t48157, t41680, t41713, t47777, t48153, t48159, t48161, t48163, t48165, t48167, t49040);
        let t49397 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2414(t42212, t42213, t47781, t47785, t47787, t49043, t49049, t49052, t49054, t49056, t49058, t49060);
        let t49409 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2415(t14363, t942, t10760, t10806, t10814, t14329, t14332, t1569, t2856, t2925, t42117, t4411, t4434, t49268, t49271, t49273, t49276, t49278, t49280, t49282, t49285, t49305, t49318, t49332, t49345, t49359, t49372, t49386, t49397, t924, t932, t952);
        let (t49426, t49450) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2416(t2929, t4446, t1568, t2886, t10737, t13520, t2860, t4408, t10770, t1561, t10524, t10720, t10734, t10743, t10747, t10753, t10756, t10765, t10772, t10805, t14263, t14439, t14443, t14450, t2863, t2906, t2930, t2933, t42020, t42149, t42226, t42228, t4437, t4449, t4454, t4472, t4475);
        let (t49485, t49488, t49491, t49492) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2417(t10811, t1568, t14255, t892, t914, t2791, t4351, t2794, t10660, t1543, t10663, t10603, t10747, t10813, t10825, t10828, t14344, t14366, t14370, t14453, t14456, t14459, t14460, t1581, t2862, t2880, t2886, t2905, t2906, t2924, t41816, t41821, t42128, t4434, t4472, t4476, t931, t950);
        let (t49496, t49499) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2418(t300, t48786, t48861, t49076, t49113, t49266, t49409, t49450, t49492, t41769, t4496, t959);
    (t49280, t49282, t49426, t49485, t49488, t49491, t49496, t49499)
}
