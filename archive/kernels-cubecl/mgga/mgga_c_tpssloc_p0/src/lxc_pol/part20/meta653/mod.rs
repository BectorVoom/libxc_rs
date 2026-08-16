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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta653<F: Float>(t10650: F, t4396: F, t13655: F, t2787: F, t10810: F, t1561: F, t47705: F, t47681: F, t47686: F, t47691: F, t47695: F, t47699: F, t47703: F, t48085: F, t48087: F, t48090: F, t48092: F, t47707: F, t48096: F, t41831: F, t41833: F, t47709: F, t47711: F, t47713: F, t47715: F, t47717: F, t47722: F, t47724: F, t47728: F, t47730: F, t41656: F, t41658: F, t41660: F, t47732: F, t47736: F, t47738: F, t47744: F, t47748: F, t48098: F, t48101: F, t48103: F, t41662: F, t41675: F, t41678: F, t41682: F, t41684: F, t41863: F, t41865: F, t41870: F, t41872: F, t41874: F, t41876: F, t48982: F, t47761: F, t47765: F, t47769: F, t48112: F, t48114: F, t48116: F, t48119: F, t48122: F, t48125: F, t48128: F, t48131: F, t41887: F, t41889: F, t48134: F, t48137: F, t48142: F, t48145: F, t48148: F, t49009: F, t49012: F, t49015: F, t49018: F, t49021: F, t48155: F, t48157: F, t41680: F, t41713: F, t47777: F, t48153: F, t48159: F, t48161: F, t48163: F, t48165: F, t48167: F, t49040: F, t42212: F, t42213: F, t47781: F, t47785: F, t47787: F, t49043: F, t49049: F, t49052: F, t49054: F, t49056: F, t49058: F, t49060: F, t14363: F, t942: F, t10760: F, t10806: F, t10814: F, t14329: F, t14332: F, t1569: F, t2856: F, t2925: F, t42117: F, t4411: F, t4434: F, t49268: F, t49271: F, t49273: F, t49276: F, t49278: F, t924: F, t932: F, t952: F, t2929: F, t4446: F, t1568: F, t2886: F, t10737: F, t13520: F, t2860: F, t4408: F, t10770: F, t10524: F, t10720: F, t10734: F, t10743: F, t10747: F, t10753: F, t10756: F, t10765: F, t10772: F, t10805: F, t14263: F, t14439: F, t14443: F, t14450: F, t2863: F, t2906: F, t2930: F, t2933: F, t42020: F, t42149: F, t42226: F, t42228: F, t4437: F, t4449: F, t4454: F, t4472: F, t4475: F, t10811: F, t14255: F, t892: F, t914: F, t2791: F, t4351: F, t2794: F, t10660: F, t1543: F, t10663: F, t10603: F, t10813: F, t10825: F, t10828: F, t14344: F, t14366: F, t14370: F, t14453: F, t14456: F, t14459: F, t14460: F, t1581: F, t2862: F, t2880: F, t2905: F, t2924: F, t41816: F, t41821: F, t42128: F, t4476: F, t931: F, t950: F, t300: F, t48786: F, t48861: F, t49076: F, t49113: F, t49266: F, t41769: F, t4496: F, t959: F) -> (F, F, F, F, F, F, F, F) {
        let (t49280, t49282, t49285, t49305) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2408::<F>(t10650, t4396, t13655, t2787, t10810, t1561, t47705, t47681, t47686, t47691, t47695, t47699, t47703, t48085, t48087, t48090, t48092);
        let t49318 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2409::<F>(t47707, t48096, t41831, t41833, t47709, t47711, t47713, t47715, t47717, t47722, t47724, t47728);
        let t49332 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2410::<F>(t47730, t41656, t41658, t41660, t47732, t47736, t47738, t47744, t47748, t48098, t48101, t48103);
        let t49345 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2411::<F>(t41662, t41675, t41678, t41682, t41684, t41863, t41865, t41870, t41872, t41874, t41876, t48982);
        let (t49359, t49372) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2412::<F>(t47761, t47765, t47769, t48112, t48114, t48116, t48119, t48122, t48125, t48128, t48131, t41887, t41889, t48134, t48137, t48142, t48145, t48148, t49009, t49012, t49015, t49018, t49021);
        let t49386 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2413::<F>(t48155, t48157, t41680, t41713, t47777, t48153, t48159, t48161, t48163, t48165, t48167, t49040);
        let t49397 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2414::<F>(t42212, t42213, t47781, t47785, t47787, t49043, t49049, t49052, t49054, t49056, t49058, t49060);
        let t49409 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2415::<F>(t14363, t942, t10760, t10806, t10814, t14329, t14332, t1569, t2856, t2925, t42117, t4411, t4434, t49268, t49271, t49273, t49276, t49278, t49280, t49282, t49285, t49305, t49318, t49332, t49345, t49359, t49372, t49386, t49397, t924, t932, t952);
        let (t49426, t49450) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2416::<F>(t2929, t4446, t1568, t2886, t10737, t13520, t2860, t4408, t10770, t1561, t10524, t10720, t10734, t10743, t10747, t10753, t10756, t10765, t10772, t10805, t14263, t14439, t14443, t14450, t2863, t2906, t2930, t2933, t42020, t42149, t42226, t42228, t4437, t4449, t4454, t4472, t4475);
        let (t49485, t49488, t49491, t49492) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2417::<F>(t10811, t1568, t14255, t892, t914, t2791, t4351, t2794, t10660, t1543, t10663, t10603, t10747, t10813, t10825, t10828, t14344, t14366, t14370, t14453, t14456, t14459, t14460, t1581, t2862, t2880, t2886, t2905, t2906, t2924, t41816, t41821, t42128, t4434, t4472, t4476, t931, t950);
        let (t49496, t49499) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2418::<F>(t300, t48786, t48861, t49076, t49113, t49266, t49409, t49450, t49492, t41769, t4496, t959);
    (t49280, t49282, t49426, t49485, t49488, t49491, t49496, t49499)
}
