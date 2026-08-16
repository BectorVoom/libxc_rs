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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta778<F: Float>(t12189: F, t6358: F, t16081: F, t19795: F, t1307: F, t54718: F, t56463: F, t686: F, t16094: F, t16095: F, t5187: F, t56467: F, t1315: F, t16101: F, t19631: F, t19781: F, t19793: F, t210: F, t213: F, t214: F, t221: F, t3719: F, t3733: F, t3734: F, t40372: F, t5195: F, t54728: F, t56275: F, t56482: F, t56484: F, t56486: F, t19767: F, t40409: F, t19771: F, t3726: F, t12199: F, t19775: F, t40387: F, t40401: F, t40402: F, t40404: F, t40407: F, t40410: F, t40422: F, t40425: F, t54663: F, t54667: F, t54671: F, t19783: F, t54670: F, t19787: F, t5308: F, t16018: F, t46838: F, t5196: F, t54673: F, t54676: F, t54690: F, t54698: F, t54701: F, t54705: F, t54711: F, t54721: F, t54725: F, t56475: F, t20032: F, t225: F, t20040: F, t12033: F, t1386: F, t16022: F, t16437: F, t16452: F, t16453: F, t16475: F, t1843: F, t20023: F, t20029: F, t20044: F, t20060: F, t26224: F, t3752: F, t3882: F, t3889: F, t3912: F, t5215: F, t5321: F, t5354: F, t55093: F, t55118: F, t562: F, t568: F, t6434: F, t6440: F, t6461: F, t19635: F, t20048: F, t1375: F, t16030: F, t16122: F, t16436: F, t16460: F, t16471: F, t1834: F, t1842: F, t19648: F, t20026: F, t3758: F, t3879: F, t3887: F, t3888: F, t3911: F, t40591: F, t5210: F, t5318: F, t5326: F, t6361: F, t6439: F, t6460: F, t3791: F, t40046: F, t16398: F, t20004: F, t19945: F, t120: F, t1352: F, t16048: F, t16233: F, t16242: F, t19871: F, t19989: F, t3803: F, t3805: F, t5248: F, t5249: F, t53881: F, t53883: F, t53893: F, t53895: F, t53897: F, t53901: F, t53903: F, t53907: F, t53917: F, t53919: F, t54744: F, t550: F, t19966: F, t5259: F, t53945: F, t119: F, t12419: F, t16148: F, t16305: F, t16314: F, t16401: F, t19873: F, t19876: F, t19979: F, t19984: F, t20468: F, t3793: F, t39936: F, t39948: F, t39950: F, t40168: F, t5246: F, t5301: F, t53921: F, t53927: F, t53929: F, t53946: F, t53965: F, t53973: F, t54013: F, t54014: F, t54258: F, t54614: F, t19844: F, t1831: F, t53906: F, t12420: F, t12429: F, t16224: F, t16312: F, t16333: F, t19894: F, t19956: F, t20473: F, t3851: F, t5240: F, t5287: F, t53984: F, t53997: F, t54003: F, t54034: F, t54043: F, t16336: F, t5314: F, t53880: F, t19930: F, t3866: F, t1351: F, t6414: F, t19731: F, t12336: F, t1363: F, t1367: F, t16227: F, t16248: F, t16311: F, t16321: F, t16394: F, t19958: F, t3783: F, t3807: F, t5250: F, t53910: F, t54047: F, t54059: F, t6427: F, t6431: F, t820: F, t12283: F, t19976: F, t19886: F, t16257: F, t16261: F, t16306: F, t3856: F, t54086: F, t54088: F, t54090: F, t54092: F, t54114: F, t54116: F, t54118: F, t54162: F, t54165: F, t6394: F, t19815: F, t3802: F, t20000: F, t54566: F, t16397: F, t5234: F, t5252: F, t16244: F, t16265: F, t16383: F, t19986: F, t19991: F, t3809: F, t39993: F, t53958: F, t54125: F, t54131: F, t54133: F, t54135: F, t54138: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t56491, t56493, t56501, t56505, t56514) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2690::<F>(t12189, t6358, t16081, t19795, t1307, t54718, t56463, t686, t16094, t16095, t5187, t56467);
        let t56525 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2691::<F>(t1307, t1315, t16101, t19631, t19781, t19793, t210, t213, t214, t221, t3719, t3733, t3734, t40372, t5195, t54728, t56275, t56482, t56484, t56486, t56491, t56493, t56501, t56505, t56514);
        let t56542 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2692::<F>(t19767, t40409, t19771, t3726, t12199, t19775, t40387, t40401, t40402, t40404, t40407, t40410, t40422, t40425, t54663, t54667, t54671);
        let (t56560, t56568) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2693::<F>(t19783, t54670, t16081, t19787, t5187, t5308, t16018, t16101, t19781, t221, t3719, t46838, t5195, t5196, t54673, t54676, t54690, t54698, t54701, t54705, t54711, t54721, t54725);
        let (t56570, t56605) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2694::<F>(t56475, t56525, t56542, t56568, t20032, t225, t20040, t12033, t1386, t16022, t16437, t16452, t16453, t16475, t1843, t20023, t20029, t20044, t20060, t26224, t3752, t3882, t3889, t3912, t5215, t5321, t5354, t55093, t55118, t562, t568, t6434, t6440, t6461);
        let t56649 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2695::<F>(t19635, t225, t20048, t1375, t1386, t16022, t16030, t16122, t16436, t16460, t16471, t16475, t1834, t1842, t19648, t20026, t3758, t3879, t3882, t3887, t3888, t3911, t40591, t5210, t5215, t5318, t5321, t5326, t5354, t568, t6361, t6439, t6460);
        let (t56666, t56689) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2696::<F>(t3791, t40046, t16398, t20004, t19945, t120, t1352, t16018, t16048, t16233, t16242, t19631, t19871, t19989, t3803, t3805, t5248, t5249, t53881, t53883, t53893, t53895, t53897, t53901, t53903, t53907, t53917, t53919, t54744, t550);
        let t56729 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2697::<F>(t16398, t19966, t5259, t53945, t119, t12419, t1315, t16148, t16233, t16305, t16314, t16401, t19873, t19876, t19979, t19984, t20468, t210, t3793, t3805, t39936, t39948, t39950, t40168, t5246, t5301, t53921, t53927, t53929, t53946, t53965, t53973, t54013, t54014, t54258, t54614, t56275);
        let t56778 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2698::<F>(t19844, t3726, t1831, t53906, t12419, t12420, t12429, t16048, t16224, t16233, t16305, t16312, t16333, t16401, t19871, t19894, t19945, t19956, t19979, t19984, t20473, t3793, t3803, t3805, t3851, t5240, t5246, t5248, t5287, t5308, t53984, t53997, t54003, t54034, t54043);
        let (t56817, t56826) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2699::<F>(t16336, t5314, t1831, t53880, t19930, t3866, t1351, t5187, t6414, t120, t19731, t12336, t12429, t1363, t1367, t16227, t16248, t16305, t16311, t16321, t16394, t19871, t19958, t3783, t3793, t3803, t3807, t5246, t5248, t5250, t53910, t54047, t54059, t56275, t6427, t6431, t820);
        let t56866 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2700::<F>(t12283, t19976, t19886, t16257, t16261, t16305, t16306, t16311, t19876, t19956, t19984, t3803, t3805, t3856, t5246, t5248, t5259, t5287, t54013, t54086, t54088, t54090, t54092, t54114, t54116, t54118, t54162, t54165, t54258, t6394);
        let t56904 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2701::<F>(t19815, t3802, t20000, t54566, t16398, t19873, t16397, t5234, t5252, t12429, t16244, t16265, t16383, t16394, t16401, t19871, t19966, t19986, t19991, t20004, t3803, t3805, t3809, t39993, t5246, t53958, t54125, t54131, t54133, t54135, t54138, t6394);
    (t56560, t56570, t56605, t56649, t56666, t56689, t56729, t56778, t56817, t56826, t56866, t56904)
}
