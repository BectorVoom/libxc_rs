//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta783 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2679;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2680;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2681;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2682;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2683;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2684;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2685;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2686;
use chunk8::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2687;
use chunk9::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2688;
use chunk10::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2689;
use chunk11::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2690;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta783(t1799: f64, t5286: f64, t16224: f64, t16305: f64, t1825: f64, t19919: f64, t19924: f64, t3803: f64, t40006: f64, t40060: f64, t54063: f64, t57007: f64, t57009: f64, t57011: f64, t57019: f64, t57022: f64, t57041: f64, t57057: f64, t57071: f64, t57073: f64, t1315: f64, t210: f64, t214: f64, t40343: f64, t40347: f64, t40350: f64, t54631: f64, t54633: f64, t54638: f64, t54639: f64, t54644: f64, t56465: f64, t56469: f64, t74355: f64, t118: f64, t20416: f64, t3739: f64, t794: f64, t16094: f64, t16095: f64, t6347: f64, t686: f64, t213: f64, t1307: f64, t16084: f64, t16101: f64, t19631: f64, t19781: f64, t20356: f64, t221: f64, t40351: f64, t5187: f64, t5195: f64, t5196: f64, t54728: f64, t56482: f64, t56484: f64, t56491: f64, t56493: f64, t20582: f64, t40021: f64, t40412: f64, t20576: f64, t3726: f64, t40372: f64, t40401: f64, t40402: f64, t40407: f64, t46838: f64, t56501: f64, t56505: f64, t56514: f64, t74389: f64, t16081: f64, t20586: f64, t40422: f64, t54663: f64, t54668: f64, t54676: f64, t54702: f64, t54725: f64, t56535: f64, t56537: f64, t56539: f64, t56548: f64, t56550: f64, t225: f64, t16311: f64, t19876: f64, t19890: f64, t19966: f64, t40124: f64, t40145: f64, t5246: f64, t54534: f64, t554: f64, t559: f64, t57127: f64, t57143: f64, t57145: f64, t57158: f64, t57160: f64, t57170: f64, t6414: f64, t1352: f64, t16306: f64, t20448: f64, t20563: f64, t54556: f64, t54582: f64, t54612: f64, t57308: f64, t57310: f64, t57324: f64, t57383: f64, t57392: f64, t57396: f64, t57398: f64, t57407: f64, t57409: f64, t16233: f64, t16394: f64, t19886: f64, t19894: f64, t19981: f64, t40449: f64, t54013: f64, t54014: f64, t54786: f64, t54793: f64, t54812: f64, t56812: f64, t57091: f64, t57437: f64, t57450: f64, t57457: f64, t6394: f64, t74415: f64, t74133: f64, t74181: f64, t74216: f64, t74253: f64, t74286: f64, t74316: f64, t74386: f64, t74428: f64, t74569: f64, t74610: f64, t74632: f64, t74655: f64, t20602: f64, t20420: f64, t1323: f64, t1375: f64, t1385: f64, t1386: f64, t16030: f64, t16439: f64, t1807: f64, t1843: f64, t20009: f64, t20023: f64, t20025: f64, t20601: f64, t20661: f64, t20662: f64, t26224: f64, t3882: f64, t3887: f64, t5215: f64, t539: f64, t55118: f64, t56596: f64, t56607: f64, t568: f64, t6440: f64, t6461: f64, t16022: f64, t20026: f64, t20029: f64, t20051: f64, t20608: f64, t20613: f64, t3758: f64, t40591: f64, t5318: f64, t5321: f64, t5353: f64, t5354: f64, t56422: f64, t6361: f64, t6460: f64, t20672: f64, t1372: f64, t16460: f64, t20044: f64, t20060: f64, t20594: f64, t20609: f64, t5210: f64, t5326: f64, t562: f64, t56434: f64, t56580: f64, t6434: f64) -> (f64, f64, f64, f64, f64) {
        let (t74677, t74682) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2679(t1799, t5286, t16224, t16305, t1825, t19919, t19924, t3803, t40006, t40060, t54063, t57007, t57009, t57011, t57019, t57022, t57041, t57057, t57071, t57073);
        let t74699 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2680(t1315, t210, t214, t40343, t40347, t40350, t54631, t54633, t54638, t54639, t54644, t56465, t56469, t74355);
        let t74735 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2681(t118, t20416, t3739, t794, t16094, t16095, t6347, t686, t213, t1307, t16084, t16101, t19631, t19781, t20356, t221, t40351, t5187, t5195, t5196, t54728, t56482, t56484, t56491, t56493);
        let t74754 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2682(t20582, t40021, t118, t20356, t40412, t794, t20576, t3726, t16101, t40372, t40401, t40402, t40407, t46838, t56501, t56505, t56514, t74389);
        let t74765 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2683(t16081, t20586, t40422, t54663, t54668, t54676, t54702, t54725, t56535, t56537, t56539, t56548, t56550);
        let (t74767, t74768, t74786) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2684(t74699, t74735, t74754, t74765, t225, t1307, t16305, t16311, t19876, t19890, t19966, t40124, t40145, t5246, t54534, t554, t559, t57127, t57143, t57145, t57158, t57160, t57170, t6414, t74677);
        let t74806 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2685(t1352, t16224, t16306, t20448, t20563, t3803, t54556, t54582, t54612, t57308, t57310, t57324, t57383, t57392, t57396, t57398, t57407, t57409);
        let t74833 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2686(t1352, t16233, t16305, t16394, t19886, t19894, t19981, t3803, t40449, t54013, t54014, t54786, t54793, t54812, t56812, t57091, t57437, t57450, t57457, t6394, t74415);
        let t74837 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2687(t74133, t74181, t74216, t74253, t74286, t74316, t74386, t74428, t74569, t74610, t74632, t74655, t74682, t74786, t74806, t74833);
        let t74868 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2688(t20602, t225, t20420, t1323, t1375, t1385, t1386, t16030, t16439, t1807, t1843, t20009, t20023, t20025, t20601, t20661, t20662, t26224, t3882, t3887, t5215, t539, t55118, t56596, t56607, t568, t6440, t6461, t74837);
        let t74899 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2689(t1375, t1385, t16022, t16030, t16439, t1843, t20023, t20026, t20029, t20051, t20608, t20613, t20662, t3758, t3887, t40591, t5215, t5318, t5321, t5353, t5354, t56422, t568, t6361, t6440, t6460, t6461);
        let t74929 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2690(t20672, t225, t1372, t1386, t16022, t16460, t1843, t20044, t20060, t20594, t20609, t20613, t3758, t3882, t5210, t5326, t562, t56434, t56580, t568, t6434, t6440, t6461, t74767);
    (t74768, t74837, t74868, t74899, t74929)
}
