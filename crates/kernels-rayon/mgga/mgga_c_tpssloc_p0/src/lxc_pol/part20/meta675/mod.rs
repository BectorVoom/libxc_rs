//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta675 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2544;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2545;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2546;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2547;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2548;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2549;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta675(t43959: f64, t4786: f64, t11424: f64, t15051: f64, t11185: f64, t15061: f64, t1117: f64, t14914: f64, t3264: f64, t11350: f64, t1682: f64, t1136: f64, t15118: f64, t3332: f64, t44131: f64, t51453: f64, t51456: f64, t51459: f64, t51463: f64, t51466: f64, t51470: f64, t51472: f64, t51474: f64, t51476: f64, t11352: f64, t4819: f64, t11303: f64, t11306: f64, t11344: f64, t11415: f64, t11420: f64, t11430: f64, t15117: f64, t15136: f64, t15156: f64, t15159: f64, t15164: f64, t15165: f64, t15168: f64, t15171: f64, t15172: f64, t1683: f64, t3333: f64, t3351: f64, t3357: f64, t3359: f64, t44172: f64, t44177: f64, t44179: f64, t44214: f64, t44361: f64, t4820: f64, t4823: f64, t11190: f64, t11191: f64, t1671: f64, t50826: f64, t50919: f64, t43727: f64, t43729: f64, t43748: f64, t43750: f64, t50828: f64, t50832: f64, t50834: f64, t50897: f64, t50900: f64, t50903: f64, t50905: f64, t50907: f64, t50912: f64, t50917: f64, t50921: f64, t50926: f64, t50931: f64, t50934: f64, t50948: f64, t43780: f64, t43782: f64, t43784: f64, t43786: f64, t43788: f64, t43816: f64, t44348: f64, t50937: f64, t50940: f64, t50946: f64, t50950: f64, t50952: f64, t50954: f64, t50957: f64, t50961: f64, t50966: f64, t50994: f64, t51000: f64, t51004: f64, t423: f64, t1128: f64, t15204: f64, t3356: f64, t4794: f64, t11349: f64, t1675: f64, t14829: f64, t3403: f64, t11297: f64, t11345: f64, t11353: f64, t1138: f64, t11434: f64, t1155: f64, t15126: f64, t15141: f64, t15179: f64, t15182: f64, t15185: f64, t3352: f64, t3360: f64, t3401: f64, t44202: f64, t44205: f64, t44295: f64, t44300: f64, t4797: f64, t4824: f64, t4840: f64, t11275: f64, t1670: f64, t43976: f64, t11285: f64, t4857: f64, t11129: f64, t11310: f64, t11365: f64, t11399: f64, t11437: f64, t11441: f64, t15133: f64, t15146: f64, t15153: f64, t15207: f64, t15218: f64, t15225: f64, t1694: f64, t1695: f64, t3376: f64, t3377: f64, t3395: f64, t43692: f64, t44155: f64, t44223: f64, t4858: f64, t4861: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t51478, t51480, t51482, t51485, t51493) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2544(t43959, t4786, t11424, t15051, t11185, t15061, t1117, t14914, t3264, t11350, t1682, t1136, t15118, t3332, t44131, t51453, t51456, t51459, t51463, t51466, t51470, t51472, t51474, t51476);
        let t51538 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2545(t11352, t4819, t11303, t11306, t11344, t11350, t1136, t11415, t11420, t11430, t15117, t15136, t15156, t15159, t15164, t15165, t15168, t15171, t15172, t1682, t1683, t3332, t3333, t3351, t3357, t3359, t44172, t44177, t44179, t44214, t44361, t4820, t4823);
        let (t51549, t51570) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2546(t11190, t11191, t1671, t50826, t50919, t43727, t43729, t43748, t43750, t50828, t50832, t50834, t50897, t50900, t50903, t50905, t50907, t50912, t50917, t50921, t50926, t50931, t50934);
        let t51590 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2547(t50948, t43780, t43782, t43784, t43786, t43788, t43816, t44348, t50937, t50940, t50946, t50950, t50952, t50954, t50957, t50961, t50966, t50994, t51000, t51004);
        let (t51593, t51617) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2548(t423, t51570, t51590, t1128, t15204, t3356, t4794, t11349, t1675, t14829, t3403, t11297, t11345, t11353, t1138, t11434, t1155, t15126, t15141, t15179, t15182, t15185, t1683, t3352, t3360, t3401, t44202, t44205, t44295, t44300, t4797, t4824, t4840, t51549);
        let (t51641, t51664) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2549(t11275, t1670, t1117, t43976, t11285, t4857, t11129, t11303, t11310, t11365, t11399, t11437, t11441, t1155, t15133, t15146, t15153, t15207, t15218, t15225, t1694, t1695, t3376, t3377, t3395, t3401, t43692, t44155, t44223, t4858, t4861);
    (t51478, t51480, t51482, t51485, t51493, t51538, t51549, t51593, t51617, t51641, t51664)
}
