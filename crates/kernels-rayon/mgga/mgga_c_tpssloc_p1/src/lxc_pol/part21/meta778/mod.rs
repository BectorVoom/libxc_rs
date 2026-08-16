//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta778 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2690;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2691;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2692;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2693;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2694;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2695;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2696;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2697;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2698;
use chunk9::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2699;
use chunk10::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2700;
use chunk11::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2701;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta778(t12189: f64, t6358: f64, t16081: f64, t19795: f64, t1307: f64, t54718: f64, t56463: f64, t686: f64, t16094: f64, t16095: f64, t5187: f64, t56467: f64, t1315: f64, t16101: f64, t19631: f64, t19781: f64, t19793: f64, t210: f64, t213: f64, t214: f64, t221: f64, t3719: f64, t3733: f64, t3734: f64, t40372: f64, t5195: f64, t54728: f64, t56275: f64, t56482: f64, t56484: f64, t56486: f64, t19767: f64, t40409: f64, t19771: f64, t3726: f64, t12199: f64, t19775: f64, t40387: f64, t40401: f64, t40402: f64, t40404: f64, t40407: f64, t40410: f64, t40422: f64, t40425: f64, t54663: f64, t54667: f64, t54671: f64, t19783: f64, t54670: f64, t19787: f64, t5308: f64, t16018: f64, t46838: f64, t5196: f64, t54673: f64, t54676: f64, t54690: f64, t54698: f64, t54701: f64, t54705: f64, t54711: f64, t54721: f64, t54725: f64, t56475: f64, t20032: f64, t225: f64, t20040: f64, t12033: f64, t1386: f64, t16022: f64, t16437: f64, t16452: f64, t16453: f64, t16475: f64, t1843: f64, t20023: f64, t20029: f64, t20044: f64, t20060: f64, t26224: f64, t3752: f64, t3882: f64, t3889: f64, t3912: f64, t5215: f64, t5321: f64, t5354: f64, t55093: f64, t55118: f64, t562: f64, t568: f64, t6434: f64, t6440: f64, t6461: f64, t19635: f64, t20048: f64, t1375: f64, t16030: f64, t16122: f64, t16436: f64, t16460: f64, t16471: f64, t1834: f64, t1842: f64, t19648: f64, t20026: f64, t3758: f64, t3879: f64, t3887: f64, t3888: f64, t3911: f64, t40591: f64, t5210: f64, t5318: f64, t5326: f64, t6361: f64, t6439: f64, t6460: f64, t3791: f64, t40046: f64, t16398: f64, t20004: f64, t19945: f64, t120: f64, t1352: f64, t16048: f64, t16233: f64, t16242: f64, t19871: f64, t19989: f64, t3803: f64, t3805: f64, t5248: f64, t5249: f64, t53881: f64, t53883: f64, t53893: f64, t53895: f64, t53897: f64, t53901: f64, t53903: f64, t53907: f64, t53917: f64, t53919: f64, t54744: f64, t550: f64, t19966: f64, t5259: f64, t53945: f64, t119: f64, t12419: f64, t16148: f64, t16305: f64, t16314: f64, t16401: f64, t19873: f64, t19876: f64, t19979: f64, t19984: f64, t20468: f64, t3793: f64, t39936: f64, t39948: f64, t39950: f64, t40168: f64, t5246: f64, t5301: f64, t53921: f64, t53927: f64, t53929: f64, t53946: f64, t53965: f64, t53973: f64, t54013: f64, t54014: f64, t54258: f64, t54614: f64, t19844: f64, t1831: f64, t53906: f64, t12420: f64, t12429: f64, t16224: f64, t16312: f64, t16333: f64, t19894: f64, t19956: f64, t20473: f64, t3851: f64, t5240: f64, t5287: f64, t53984: f64, t53997: f64, t54003: f64, t54034: f64, t54043: f64, t16336: f64, t5314: f64, t53880: f64, t19930: f64, t3866: f64, t1351: f64, t6414: f64, t19731: f64, t12336: f64, t1363: f64, t1367: f64, t16227: f64, t16248: f64, t16311: f64, t16321: f64, t16394: f64, t19958: f64, t3783: f64, t3807: f64, t5250: f64, t53910: f64, t54047: f64, t54059: f64, t6427: f64, t6431: f64, t820: f64, t12283: f64, t19976: f64, t19886: f64, t16257: f64, t16261: f64, t16306: f64, t3856: f64, t54086: f64, t54088: f64, t54090: f64, t54092: f64, t54114: f64, t54116: f64, t54118: f64, t54162: f64, t54165: f64, t6394: f64, t19815: f64, t3802: f64, t20000: f64, t54566: f64, t16397: f64, t5234: f64, t5252: f64, t16244: f64, t16265: f64, t16383: f64, t19986: f64, t19991: f64, t3809: f64, t39993: f64, t53958: f64, t54125: f64, t54131: f64, t54133: f64, t54135: f64, t54138: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t56491, t56493, t56501, t56505, t56514) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2690(t12189, t6358, t16081, t19795, t1307, t54718, t56463, t686, t16094, t16095, t5187, t56467);
        let t56525 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2691(t1307, t1315, t16101, t19631, t19781, t19793, t210, t213, t214, t221, t3719, t3733, t3734, t40372, t5195, t54728, t56275, t56482, t56484, t56486, t56491, t56493, t56501, t56505, t56514);
        let t56542 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2692(t19767, t40409, t19771, t3726, t12199, t19775, t40387, t40401, t40402, t40404, t40407, t40410, t40422, t40425, t54663, t54667, t54671);
        let (t56560, t56568) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2693(t19783, t54670, t16081, t19787, t5187, t5308, t16018, t16101, t19781, t221, t3719, t46838, t5195, t5196, t54673, t54676, t54690, t54698, t54701, t54705, t54711, t54721, t54725);
        let (t56570, t56605) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2694(t56475, t56525, t56542, t56568, t20032, t225, t20040, t12033, t1386, t16022, t16437, t16452, t16453, t16475, t1843, t20023, t20029, t20044, t20060, t26224, t3752, t3882, t3889, t3912, t5215, t5321, t5354, t55093, t55118, t562, t568, t6434, t6440, t6461);
        let t56649 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2695(t19635, t225, t20048, t1375, t1386, t16022, t16030, t16122, t16436, t16460, t16471, t16475, t1834, t1842, t19648, t20026, t3758, t3879, t3882, t3887, t3888, t3911, t40591, t5210, t5215, t5318, t5321, t5326, t5354, t568, t6361, t6439, t6460);
        let (t56666, t56689) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2696(t3791, t40046, t16398, t20004, t19945, t120, t1352, t16018, t16048, t16233, t16242, t19631, t19871, t19989, t3803, t3805, t5248, t5249, t53881, t53883, t53893, t53895, t53897, t53901, t53903, t53907, t53917, t53919, t54744, t550);
        let t56729 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2697(t16398, t19966, t5259, t53945, t119, t12419, t1315, t16148, t16233, t16305, t16314, t16401, t19873, t19876, t19979, t19984, t20468, t210, t3793, t3805, t39936, t39948, t39950, t40168, t5246, t5301, t53921, t53927, t53929, t53946, t53965, t53973, t54013, t54014, t54258, t54614, t56275);
        let t56778 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2698(t19844, t3726, t1831, t53906, t12419, t12420, t12429, t16048, t16224, t16233, t16305, t16312, t16333, t16401, t19871, t19894, t19945, t19956, t19979, t19984, t20473, t3793, t3803, t3805, t3851, t5240, t5246, t5248, t5287, t5308, t53984, t53997, t54003, t54034, t54043);
        let (t56817, t56826) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2699(t16336, t5314, t1831, t53880, t19930, t3866, t1351, t5187, t6414, t120, t19731, t12336, t12429, t1363, t1367, t16227, t16248, t16305, t16311, t16321, t16394, t19871, t19958, t3783, t3793, t3803, t3807, t5246, t5248, t5250, t53910, t54047, t54059, t56275, t6427, t6431, t820);
        let t56866 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2700(t12283, t19976, t19886, t16257, t16261, t16305, t16306, t16311, t19876, t19956, t19984, t3803, t3805, t3856, t5246, t5248, t5259, t5287, t54013, t54086, t54088, t54090, t54092, t54114, t54116, t54118, t54162, t54165, t54258, t6394);
        let t56904 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2701(t19815, t3802, t20000, t54566, t16398, t19873, t16397, t5234, t5252, t12429, t16244, t16265, t16383, t16394, t16401, t19871, t19966, t19986, t19991, t20004, t3803, t3805, t3809, t39993, t5246, t53958, t54125, t54131, t54133, t54135, t54138, t6394);
    (t56560, t56570, t56605, t56649, t56666, t56689, t56729, t56778, t56817, t56826, t56866, t56904)
}
