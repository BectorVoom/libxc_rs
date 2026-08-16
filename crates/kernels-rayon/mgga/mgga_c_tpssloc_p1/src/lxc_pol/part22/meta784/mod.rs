//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta784 (260520-c91 hierarchical CSE).
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
mod chunk11;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2691;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2692;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2693;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2694;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2695;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2696;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2697;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2698;
use chunk8::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2699;
use chunk9::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2700;
use chunk10::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2701;
use chunk11::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2702;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta784(t20670: f64, t225: f64, t1834: f64, t6414: f64, t6387: f64, t20553: f64, t562: f64, t20489: f64, t16036: f64, t16047: f64, t16055: f64, t1825: f64, t19654: f64, t19661: f64, t19735: f64, t19743: f64, t19744: f64, t19810: f64, t20018: f64, t20473: f64, t20638: f64, t5250: f64, t5287: f64, t5333: f64, t5334: f64, t5336: f64, t5344: f64, t54963: f64, t57704: f64, t6378: f64, t74599: f64, t12250: f64, t1824: f64, t6434: f64, t1336: f64, t1352: f64, t19657: f64, t19748: f64, t19815: f64, t20490: f64, t20568: f64, t20622: f64, t3777: f64, t3901: f64, t40492: f64, t5335: f64, t5349: f64, t57618: f64, t74174: f64, t12171: f64, t16060: f64, t16132: f64, t1840: f64, t19658: f64, t19660: f64, t19752: f64, t19805: f64, t20495: f64, t20648: f64, t26322: f64, t5234: f64, t5339: f64, t5341: f64, t6420: f64, t6454: f64, t1338: f64, t20601: f64, t16040: f64, t19668: f64, t19732: f64, t20625: f64, t20643: f64, t5348: f64, t5351: f64, t57659: f64, t6415: f64, t6448: f64, t6451: f64, t12021: f64, t1332: f64, t1351: f64, t1375: f64, t1378: f64, t1380: f64, t1381: f64, t1383: f64, t1386: f64, t16033: f64, t1814: f64, t1838: f64, t1842: f64, t1843: f64, t19648: f64, t19674: f64, t19733: f64, t19736: f64, t19739: f64, t19740: f64, t19745: f64, t19756: f64, t19761: f64, t19763: f64, t19804: f64, t19813: f64, t20010: f64, t20014: f64, t20022: f64, t20026: f64, t20029: f64, t20044: f64, t20051: f64, t20060: f64, t20554: f64, t20595: f64, t20609: f64, t20616: f64, t20630: f64, t20632: f64, t20635: f64, t20645: f64, t20651: f64, t26318: f64, t3882: f64, t3887: f64, t5215: f64, t5230: f64, t5321: f64, t5326: f64, t5343: f64, t5345: f64, t5353: f64, t5354: f64, t544: f64, t54905: f64, t553: f64, t564: f64, t56640: f64, t568: f64, t56923: f64, t57530: f64, t57568: f64, t6388: f64, t6439: f64, t6456: f64, t6458: f64, t74289: f64, t74564: f64, t74768: f64, t74837: f64, t1390: f64, t16497: f64, t193: f64, t19577: f64, t19631: f64, t20063: f64, t20067: f64, t20077: f64, t20085: f64, t3918: f64, t39483: f64, t5122: f64, t5126: f64, t5160: f64, t5161: f64, t5308: f64, t533: f64, t54409: f64, t6330: f64, t74086: f64, t74470: f64, t74868: f64, t74899: f64, t74929: f64, t1388: f64, t6463: f64, t1307: f64, t15899: f64, t20563: f64, t3919: f64, t39529: f64, t39539: f64, t39549: f64, t74476: f64, t74477: f64, t74478: f64, t74479: f64, t19924: f64, t19994: f64, t39585: f64, t39590: f64, t39593: f64, t39595: f64, t54431: f64, t54436: f64, t74484: f64, t74485: f64, t74486: f64, t12461: f64, t20684: f64, t39655: f64, t39658: f64, t39844: f64, t5356: f64, t54453: f64, t74490: f64, t74491: f64, t74493: f64, t74494: f64, t74496: f64, t74497: f64, t571: f64, t1297: f64, t40224: f64, t40230: f64, t54470: f64, t54472: f64, t54473: f64, t54475: f64, t54478: f64, t74355: f64, t74502: f64, t74503: f64, t74504: f64, t4025: f64, t5456: f64, t20193: f64, t604: f64, t1411: f64, t1434: f64, t19322: f64, t19363: f64, t19441: f64, t20207: f64, t20264: f64, t20285: f64, t3962: f64, t3966: f64, t3968: f64, t3971: f64, t3976: f64, t5398: f64, t5442: f64, t55653: f64, t608: f64, t609: f64, t65: f64, t6509: f64, t67: f64, t80: f64, t1409: f64, t1426: f64, t16558: f64, t17635: f64, t1864: f64, t19323: f64, t19331: f64, t19334: f64, t20218: f64, t20219: f64, t20222: f64, t31: f64, t3997: f64, t5399: f64, t628: f64, t642: f64, t67060: f64, t70458: f64, t7445: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t74930, t74937, t74941, t74949, t74967, t74996) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2691(t20670, t225, t1834, t6414, t6387, t20553, t562, t20489, t16036, t16047, t16055, t1825, t19654, t19661, t19735, t19743, t19744, t19810, t20018, t20473, t20638, t5250, t5287, t5333, t5334, t5336, t5344, t54963, t57704, t6378, t74599);
        let (t75008, t75053) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2692(t12250, t6414, t1824, t6434, t1336, t1352, t16047, t1825, t19654, t19657, t19744, t19748, t19815, t20490, t20568, t20622, t3777, t3901, t40492, t5250, t5287, t5334, t5335, t5344, t5349, t57618, t74174, t74941);
        let t75101 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2693(t12171, t1336, t1352, t16060, t16132, t1840, t19658, t19660, t19752, t19805, t19815, t20495, t20648, t26322, t3777, t5234, t5339, t5341, t5344, t6420, t6454, t74967);
        let t75150 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2694(t1338, t20601, t1336, t1352, t16040, t16060, t16132, t1825, t19668, t19732, t20473, t20625, t20643, t3777, t5234, t5334, t5348, t5351, t57659, t6378, t6415, t6448, t6451);
        let t75183 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2695(t12021, t1332, t1336, t1351, t1352, t1375, t1378, t1380, t1381, t1383, t1386, t16033, t16047, t16060, t1814, t1834, t1838, t1842, t1843, t19648, t19654, t19660, t19674, t19733, t19735, t19736, t19739, t19740, t19743, t19745, t19756, t19761, t19763, t19804, t19810, t19813, t20010, t20014, t20022, t20026, t20029, t20044, t20051, t20060, t20554, t20595, t20609, t20616, t20630, t20632, t20635, t20645, t20651, t26318, t3777, t3882, t3887, t3901, t5215, t5230, t5234, t5250, t5287, t5321, t5326, t5334, t5335, t5343, t5344, t5345, t5353, t5354, t544, t54905, t553, t564, t56640, t568, t56923, t57530, t57568, t6378, t6388, t6439, t6456, t6458, t74289, t74564, t74768, t74837, t74930, t74937, t74941, t74949, t74996, t75008, t75053, t75101, t75150);
        let t75198 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2696(t1390, t16497, t193, t19577, t19631, t20063, t20067, t20077, t20085, t3918, t39483, t5122, t5126, t5160, t5161, t5308, t533, t54409, t6330, t74086, t74470, t74868, t74899, t74929, t75183);
        let t75218 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2697(t1388, t6330, t6463, t1307, t15899, t20563, t3918, t3919, t39529, t39539, t39549, t5126, t5160, t5161, t74476, t74477, t74478, t74479);
        let t75237 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2698(t19924, t19994, t39585, t39590, t39593, t39595, t5122, t5126, t54431, t54436, t74484, t74485, t74486);
        let (t75240, t75254) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2699(t12461, t20684, t20085, t39655, t39658, t39844, t5160, t5356, t54453, t74490, t74491, t74493, t74494, t74496, t74497);
        let (t75256, t75267) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2700(t571, t6330, t1297, t193, t40224, t40230, t54470, t54472, t54473, t54475, t54478, t74355, t74502, t74503, t74504);
        let (t75275, t75284, t75356) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2701(t4025, t5456, t20193, t604, t1411, t1434, t19322, t19363, t19441, t20207, t20264, t20285, t3962, t3966, t3968, t3971, t3976, t5398, t5442, t55653, t608, t609, t65, t6509, t67, t80);
        let t75392 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2702(t1409, t1426, t67, t1434, t16558, t17635, t1864, t19322, t19323, t19331, t19334, t20218, t20219, t20222, t31, t3966, t3997, t5399, t628, t642, t65, t67060, t70458, t7445, t80);
    (t75198, t75218, t75237, t75240, t75254, t75256, t75267, t75275, t75284, t75356, t75392)
}
