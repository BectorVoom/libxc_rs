//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta786 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2726;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2727;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2728;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2729;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2730;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2731;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2732;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2733;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2734;
use chunk9::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2735;
use chunk10::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2736;
use chunk11::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2737;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta786(t16398: f64, t19890: f64, t12283: f64, t19972: f64, t225: f64, t56570: f64, t16150: f64, t16155: f64, t16233: f64, t16308: f64, t16387: f64, t16394: f64, t19855: f64, t19871: f64, t19876: f64, t19956: f64, t3803: f64, t3851: f64, t3858: f64, t40335: f64, t40443: f64, t40449: f64, t5240: f64, t5248: f64, t54764: f64, t54785: f64, t54787: f64, t54793: f64, t54801: f64, t54811: f64, t554: f64, t559: f64, t56689: f64, t56729: f64, t56778: f64, t56826: f64, t56866: f64, t56904: f64, t56952: f64, t56996: f64, t57030: f64, t57084: f64, t57133: f64, t57305: f64, t57351: f64, t57400: f64, t57447: f64, t1834: f64, t5286: f64, t12240: f64, t1352: f64, t16036: f64, t16037: f64, t16041: f64, t16047: f64, t16048: f64, t16055: f64, t16419: f64, t19654: f64, t19661: f64, t19735: f64, t19736: f64, t19739: f64, t19743: f64, t19810: f64, t3793: f64, t5334: f64, t5344: f64, t16046: f64, t1814: f64, t1824: f64, t5318: f64, t1351: f64, t16033: f64, t16049: f64, t16052: f64, t16125: f64, t19660: f64, t19740: f64, t19763: f64, t5230: f64, t5250: f64, t5335: f64, t5343: f64, t5345: f64, t54963: f64, t56666: f64, t57147: f64, t12250: f64, t12267: f64, t1336: f64, t16044: f64, t16206: f64, t19668: f64, t19732: f64, t19745: f64, t19748: f64, t19752: f64, t20018: f64, t3777: f64, t3901: f64, t3909: f64, t54976: f64, t6378: f64, t6448: f64, t1372: f64, t6387: f64, t6414: f64, t12259: f64, t1380: f64, t16060: f64, t16065: f64, t16068: f64, t16416: f64, t1825: f64, t19674: f64, t19761: f64, t5234: f64, t5333: f64, t5336: f64, t5339: f64, t5341: f64, t55039: f64, t57354: f64, t6420: f64, t3787: f64, t6434: f64, t1338: f64, t20009: f64, t1381: f64, t16133: f64, t16414: f64, t1838: f64, t19657: f64, t19815: f64, t3898: f64, t3902: f64, t5348: f64, t5351: f64, t53909: f64, t544: f64, t553: f64, t56923: f64, t1332: f64, t16127: f64, t16132: f64, t16423: f64, t19658: f64, t19813: f64, t20010: f64, t3773: f64, t3856: f64, t3905: f64, t3907: f64, t40486: f64, t5287: f64, t6388: f64, t6415: f64, t6456: f64, t6458: f64, t19731: f64, t562: f64, t1383: f64, t16136: f64, t16429: f64, t19805: f64, t20014: f64, t3897: f64, t5349: f64, t564: f64, t56914: f64, t6454: f64, t16123: f64, t16433: f64, t1840: f64, t19733: f64, t19744: f64, t19756: f64, t57300: f64, t6451: f64, t12021: f64, t12030: f64, t1375: f64, t1378: f64, t16030: f64, t16413: f64, t16437: f64, t16439: f64, t16453: f64, t16471: f64, t1807: f64, t1843: f64, t19648: f64, t20051: f64, t20060: f64, t3758: f64, t3882: f64, t3888: f64, t3889: f64, t3911: f64, t5215: f64, t5321: f64, t5326: f64, t5354: f64, t539: f64, t55134: f64, t568: f64, t6439: f64, t6440: f64, t6460: f64, t1390: f64, t16497: f64, t193: f64, t3918: f64, t39595: f64, t39615: f64, t5187: f64, t533: f64, t56411: f64, t56412: f64, t56416: f64, t56417: f64, t56457: f64, t56605: f64, t56649: f64, t57203: f64, t57204: f64, t57205: f64) -> f64 {
        let (t57465, t57481) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2726(t16398, t19890, t12283, t19972, t225, t56570, t16150, t16155, t16233, t16308, t16387, t16394, t19855, t19871, t19876, t19956, t3803, t3851, t3858, t40335, t40443, t40449, t5240, t5248, t54764, t54785, t54787, t54793, t54801, t54811, t554, t559);
        let t57485 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2727(t56689, t56729, t56778, t56826, t56866, t56904, t56952, t56996, t57030, t57084, t57133, t57305, t57351, t57400, t57447, t57481);
        let (t57499, t57526) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2728(t1834, t5286, t12240, t1352, t16036, t16037, t16041, t16047, t16048, t16055, t16419, t19654, t19661, t19735, t19736, t19739, t19743, t19810, t3793, t3851, t40335, t5334, t5344);
        let (t57545, t57564) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2729(t16046, t1814, t1824, t5318, t1351, t19735, t12240, t16033, t16047, t16048, t16049, t16052, t16055, t16125, t19654, t19660, t19740, t19743, t19763, t19810, t5230, t5250, t5334, t5335, t5343, t5345, t54963, t56666, t57147, t57499);
        let t57597 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2730(t12250, t5286, t12240, t12267, t1336, t1351, t16033, t16044, t16047, t16055, t16206, t19660, t19668, t19732, t19739, t19745, t19748, t19752, t19810, t20018, t3777, t3851, t3901, t3909, t5334, t5335, t5344, t54976, t6378, t6448);
        let (t57607, t57618, t57631) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2731(t1372, t6387, t6414, t12259, t1336, t1352, t1380, t16033, t16060, t16065, t16068, t16416, t1825, t19654, t19674, t19761, t19810, t3777, t5230, t5234, t5250, t5333, t5334, t5336, t5339, t5341, t5344, t55039, t57354, t6420);
        let t57667 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2732(t1352, t5286, t3787, t6434, t1338, t20009, t1336, t1381, t16133, t16206, t16414, t1814, t1838, t19657, t19815, t3793, t3851, t3898, t3902, t5230, t5234, t5335, t5344, t5348, t5351, t53909, t544, t553, t56923, t57485);
        let t57692 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2733(t12259, t12267, t1332, t1336, t16127, t16132, t16423, t19657, t19658, t19813, t19815, t20010, t3773, t3777, t3856, t3905, t3907, t40486, t5234, t5287, t6388, t6415, t6456, t6458);
        let (t57704, t57725) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2734(t19731, t562, t12267, t1336, t1352, t1383, t16033, t16036, t16060, t16136, t16429, t19739, t19805, t20014, t3856, t3897, t5234, t5250, t5287, t5334, t5344, t5349, t564, t56914, t57465, t57545, t57618, t6454);
        let t57760 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2735(t12267, t1336, t1352, t1380, t16047, t16048, t16123, t16433, t1840, t19660, t19733, t19743, t19744, t19756, t3777, t3793, t3856, t5234, t5250, t5334, t5344, t57300, t57607, t57704, t6451);
        let t57795 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2736(t12021, t12030, t1375, t1378, t16030, t16413, t16437, t16439, t16453, t16471, t1807, t1843, t19648, t20051, t20060, t3758, t3882, t3888, t3889, t3911, t5215, t5321, t5326, t5354, t539, t55134, t568, t57485, t57526, t57564, t57597, t57631, t57667, t57692, t57725, t57760, t6439, t6440, t6460);
        let t57801 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2737(t1390, t16497, t193, t3918, t39595, t39615, t5187, t533, t56411, t56412, t56416, t56417, t56457, t56605, t56649, t57203, t57204, t57205, t57795);
    t57801
}
