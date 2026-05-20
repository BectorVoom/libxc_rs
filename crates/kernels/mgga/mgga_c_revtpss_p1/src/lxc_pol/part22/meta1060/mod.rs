//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1060 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3771;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3772;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3773;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3774;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3775;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3776;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3777;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3778;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3779;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1060<F: Float>(t1214: F, t20950: F, t12916: F, t21165: F, t3718: F, t12809: F, t20796: F, t13045: F, t5284: F, t1248: F, t1121: F, t12855: F, t17170: F, t17396: F, t17605: F, t17690: F, t17709: F, t17710: F, t17736: F, t17744: F, t20978: F, t21037: F, t3611: F, t3626: F, t3720: F, t44484: F, t44952: F, t471: F, t5245: F, t5297: F, t5331: F, t5332: F, t56861: F, t59419: F, t59423: F, t59426: F, t71480: F, t69696: F, t69728: F, t69770: F, t69805: F, t69836: F, t69868: F, t69901: F, t69943: F, t69982: F, t70011: F, t70050: F, t70085: F, t70119: F, t70213: F, t70254: F, t70289: F, t70328: F, t70361: F, t70390: F, t70429: F, t70453: F, t70480: F, t70508: F, t70546: F, t70565: F, t70593: F, t70638: F, t70675: F, t70717: F, t70748: F, t70789: F, t70830: F, t70872: F, t70907: F, t70953: F, t70978: F, t71015: F, t71053: F, t71098: F, t71196: F, t71231: F, t71269: F, t71304: F, t71334: F, t71375: F, t71406: F, t71457: F, t71492: F, t71527: F, t71560: F, t71597: F, t71624: F, t71667: F, t71704: F, t71737: F, t71781: F, t71824: F, t71867: F, t71905: F, t71936: F, t71981: F, t72014: F, t72049: F, t12717: F, t12723: F, t16696: F, t17289: F, t1770: F, t17818: F, t17879: F, t17883: F, t17884: F, t17958: F, t20856: F, t21164: F, t21257: F, t21442: F, t21443: F, t21480: F, t21551: F, t3601: F, t3666: F, t43350: F, t45666: F, t45707: F, t45738: F, t45852: F, t460: F, t489: F, t5216: F, t5458: F, t5477: F, t5481: F, t5487: F, t59737: F, t59749: F, t69655: F, t70235: F, t73: F, t5457: F, t6695: F, t1234: F, t1280: F, t1287: F, t1291: F, t13133: F, t16775: F, t17454: F, t17846: F, t21333: F, t21484: F, t21513: F, t21610: F, t3670: F, t3755: F, t3782: F, t3783: F, t45654: F, t45659: F, t45683: F, t45715: F, t490: F, t5346: F, t5452: F, t5486: F, t59650: F, t6573: F, t6587: F, t70422: F, t71179: F, t71724: F, t72044: F, t12713: F, t1281: F, t1285: F, t17192: F, t17829: F, t17853: F, t17864: F, t17944: F, t1811: F, t20800: F, t21448: F, t21471: F, t21473: F, t21507: F, t21541: F, t21558: F, t3552: F, t3584: F, t45634: F, t45868: F, t5412: F, t5463: F, t5478: F, t59941: F, t6717: F, t6741: F, t70202: F, t70209: F, t70718: F, t1209: F, t1284: F, t20849: F, t3754: F, t12709: F, t12987: F, t16697: F, t17345: F, t17888: F, t17893: F, t17902: F, t17949: F, t17951: F, t1822: F, t21040: F, t21587: F, t21596: F, t3756: F, t45718: F, t5459: F, t57465: F, t59537: F, t59681: F, t59864: F, t59865: F, t59871: F, t59872: F, t59987: F, t60008: F, t70890: F, t11249: F, t6688: F, t12966: F, t16771: F, t17188: F, t17307: F, t17808: F, t17887: F, t17905: F, t17917: F, t21518: F, t21524: F, t21583: F, t21599: F, t5446: F, t5466: F, t57264: F, t59671: F, t59686: F, t59817: F, t60037: F, t71945: F, t3781: F, t6564: F, t3302: F, t13141: F, t1204: F, t12751: F, t12756: F, t17856: F, t17955: F, t20703: F, t20850: F, t21459: F, t21465: F, t21468: F, t21527: F, t21592: F, t21607: F, t3727: F, t3746: F, t3759: F, t3760: F, t3784: F, t6622: F, t6628: F, t3766: F, t17191: F, t5219: F, t21342: F, t473: F, t12975: F, t16695: F, t16750: F, t16757: F, t17821: F, t17840: F, t17876: F, t17880: F, t17945: F, t21452: F, t21456: F, t21500: F, t21542: F, t21562: F, t3767: F, t3769: F, t3770: F, t59705: F, t60019: F, t6723: F, t70712: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t72050, t72087, t72092) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3771::<F>(t1214, t20950, t12916, t21165, t3718, t12809, t20796, t13045, t5284, t1248, t1121, t12855, t17170, t17396, t17605, t17690, t17709, t17710, t17736, t17744, t20978, t21037, t3611, t3626, t3720, t44484, t44952, t471, t5245, t5297, t5331, t5332, t56861, t59419, t59423, t59426, t71480);
        let t72098 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3772::<F>(t69696, t69728, t69770, t69805, t69836, t69868, t69901, t69943, t69982, t70011, t70050, t70085, t70119, t70213, t70254, t70289, t70328, t70361, t70390, t70429, t70453, t70480, t70508, t70546, t70565, t70593, t70638, t70675, t70717, t70748, t70789, t70830, t70872, t70907, t70953, t70978, t71015, t71053, t71098, t71196, t71231, t71269, t71304, t71334, t71375, t71406, t71457, t71492, t71527, t71560, t71597, t71624, t71667, t71704, t71737, t71781, t71824, t71867, t71905, t71936, t71981, t72014, t72049, t72092);
        let t72140 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3773::<F>(t12717, t12723, t16696, t17289, t1770, t17818, t17879, t17883, t17884, t17958, t20856, t21164, t21257, t21442, t21443, t21480, t21551, t3601, t3666, t43350, t45666, t45707, t45738, t45852, t460, t471, t489, t5216, t5458, t5477, t5481, t5487, t59737, t59749, t69655, t70235, t72098, t73);
        let (t72165, t72187) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3774::<F>(t5245, t5457, t3601, t6695, t1234, t12717, t1280, t1287, t1291, t13133, t16775, t17289, t17454, t17846, t21333, t21484, t21513, t21610, t3666, t3670, t3755, t3782, t3783, t45654, t45659, t45683, t45715, t490, t5346, t5452, t5486, t59650, t6573, t6587, t69655, t70422, t71179, t71724, t72044, t72087, t73);
        let t72231 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3775::<F>(t1234, t12713, t12723, t1280, t1281, t1285, t1287, t17170, t17192, t17829, t17853, t17864, t17944, t1811, t20800, t21442, t21448, t21471, t21473, t21507, t21541, t21558, t3552, t3584, t45634, t45666, t45868, t5284, t5332, t5412, t5463, t5478, t59650, t59941, t6717, t6741, t70202, t70209, t70718);
        let t72276 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3776::<F>(t1209, t1284, t6695, t20849, t3754, t12709, t12987, t16697, t17345, t17883, t17888, t17893, t17902, t17949, t17951, t17958, t1822, t21040, t21473, t21480, t21587, t21596, t3601, t3755, t3756, t45718, t5459, t5486, t57465, t59537, t59681, t59864, t59865, t59871, t59872, t59987, t60008, t70235, t70890);
        let (t72303, t72315) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3777::<F>(t11249, t6688, t12717, t1287, t12966, t16771, t17188, t17307, t1770, t17808, t17829, t17887, t17905, t17917, t17951, t17958, t21518, t21524, t21583, t21599, t3670, t45707, t45718, t45738, t45852, t5446, t5466, t5486, t57264, t59671, t59686, t59817, t60037, t71945);
        let t72358 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3778::<F>(t3781, t6564, t20800, t3302, t13141, t1811, t460, t1204, t12723, t12751, t12756, t1285, t1287, t12966, t16696, t17192, t17454, t17856, t17864, t17902, t17955, t20703, t20850, t21459, t21465, t21468, t21513, t21518, t21527, t21592, t21607, t3670, t3727, t3746, t3759, t3760, t3784, t45634, t45683, t6622);
        let (t72359, t72404) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3779::<F>(t3727, t6628, t3766, t6564, t17191, t5219, t21342, t473, t1214, t1234, t12756, t12975, t16695, t16750, t16757, t17821, t17840, t17876, t17880, t17945, t21452, t21456, t21500, t21542, t21558, t21562, t3666, t3746, t3756, t3767, t3769, t3770, t460, t5245, t5412, t5459, t5466, t5486, t59705, t60019, t6723, t70712);
    (t72050, t72098, t72140, t72165, t72187, t72231, t72276, t72303, t72315, t72358, t72359, t72404)
}
