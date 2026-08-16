//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta820 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3027;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3028;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3029;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3030;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3031;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3032;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3033;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3034;
use chunk8::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3035;
use chunk9::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3036;
use chunk10::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3037;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta820(t342: f64, t378: f64, t43536: f64, t11631: f64, t43350: f64, t16558: f64, t989: f64, t1071: f64, t12166: f64, t12077: f64, t11247: f64, t1678: f64, t43346: f64, t42872: f64, t12046: f64, t1647: f64, t1082: f64, t12053: f64, t12078: f64, t12079: f64, t12116: f64, t16427: f64, t16443: f64, t16446: f64, t16520: f64, t16555: f64, t16562: f64, t3223: f64, t42359: f64, t43154: f64, t4961: f64, t53670: f64, t54983: f64, t15893: f64, t3153: f64, t16551: f64, t1043: f64, t1089: f64, t12097: f64, t12149: f64, t15957: f64, t16152: f64, t16410: f64, t16479: f64, t16523: f64, t16534: f64, t16577: f64, t19502: f64, t3043: f64, t3287: f64, t43438: f64, t43450: f64, t43520: f64, t43524: f64, t4964: f64, t4976: f64, t4988: f64, t5012: f64, t53340: f64, t53506: f64, t54026: f64, t55499: f64, t12153: f64, t4746: f64, t16237: f64, t359: f64, t1024: f64, t12119: f64, t12143: f64, t12146: f64, t12154: f64, t15670: f64, t15837: f64, t16390: f64, t16406: f64, t16499: f64, t16544: f64, t3204: f64, t3288: f64, t3291: f64, t380: f64, t42261: f64, t43357: f64, t54955: f64, t55377: f64, t999: f64, t15654: f64, t3286: f64, t16543: f64, t3046: f64, t1087: f64, t12133: f64, t12160: f64, t15780: f64, t16183: f64, t16393: f64, t16468: f64, t16488: f64, t16537: f64, t16573: f64, t16581: f64, t19603: f64, t43360: f64, t4980: f64, t4984: f64, t4995: f64, t4996: f64, t4999: f64, t55345: f64, t12066: f64, t12094: f64, t12122: f64, t12128: f64, t12168: f64, t16381: f64, t16540: f64, t16578: f64, t19608: f64, t3259: f64, t3304: f64, t3309: f64, t43443: f64, t43453: f64, t43562: f64, t43598: f64, t4866: f64, t4893: f64, t4977: f64, t4981: f64, t53792: f64, t54276: f64, t15669: f64, t1651: f64, t11804: f64, t12057: f64, t12150: f64, t12157: f64, t12167: f64, t16076: f64, t16433: f64, t16502: f64, t16506: f64, t3298: f64, t3322: f64, t43341: f64, t43378: f64, t4743: f64, t4954: f64, t4998: f64, t54474: f64, t54909: f64, t55330: f64, t73: f64, t43400: f64, t11173: f64, t12127: f64, t12132: f64, t16409: f64, t16505: f64, t16574: f64, t19526: f64, t19569: f64, t3318: f64, t357: f64, t4781: f64, t4975: f64, t1083: f64, t11940: f64, t12073: f64, t12163: f64, t15717: f64, t16405: f64, t16515: f64, t3278: f64, t43432: f64, t43504: f64, t43528: f64, t4757: f64, t5004: f64, t53865: f64, t1086: f64, t15886: f64, t3151: f64, t4930: f64, t3057: f64, t1090: f64, t12052: f64, t15609: f64, t16432: f64, t1689: f64, t3133: f64, t3299: f64, t3316: f64, t43413: f64, t43456: f64, t54064: f64, t54365: f64, t54370: f64, t55165: f64, t55550: f64, t11202: f64, t11782: f64, t11788: f64, t12080: f64, t12089: f64, t15655: f64, t16396: f64, t16399: f64, t16449: f64, t16465: f64, t16584: f64, t19579: f64, t3075: f64, t3283: f64, t3292: f64, t43446: f64, t4970: f64, t5009: f64, t54695: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t55569, t55570, t55575, t55579, t55583, t55586) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3027(t342, t378, t43536, t11631, t43350, t16558, t989, t1071, t12166, t12077, t11247, t1678);
        let t55607 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3028(t342, t378, t43346, t42872, t43350, t12046, t1647, t1082, t11247, t12053, t12078, t12079, t12116, t16427, t16443, t16446, t16520, t16555, t16562, t3223, t42359, t43154, t4961, t53670, t54983, t55569, t55570, t55575, t55579, t55583, t55586);
        let (t55612, t55643) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3029(t15893, t3153, t16551, t989, t1043, t1089, t12097, t12149, t15957, t16152, t16410, t16443, t16479, t16523, t16534, t16555, t16577, t19502, t3043, t3223, t3287, t43438, t43450, t43520, t43524, t4964, t4976, t4988, t5012, t53340, t53506, t54026, t55499);
        let t55676 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3030(t12153, t4746, t16237, t359, t1024, t1082, t12119, t12143, t12146, t12154, t15670, t15837, t16390, t16406, t16499, t16544, t3204, t3288, t3291, t342, t380, t42261, t43357, t4964, t54955, t55377, t999);
        let t55711 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3031(t15654, t3286, t16543, t3046, t1071, t1087, t1089, t12133, t12146, t12154, t12160, t15780, t16183, t16393, t16410, t16468, t16488, t16537, t16573, t16581, t19603, t3043, t3287, t3288, t43360, t4980, t4984, t4995, t4996, t4999, t55345);
        let t55746 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3032(t4746, t4995, t1087, t1089, t12066, t12079, t12094, t12122, t12128, t12168, t16381, t1647, t16540, t16578, t19608, t3259, t3304, t3309, t43357, t43443, t43453, t43520, t43524, t43562, t43598, t4866, t4893, t4977, t4981, t53792, t54276);
        let t55783 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3033(t15669, t3286, t1651, t378, t11804, t12057, t12149, t12150, t12157, t12167, t12168, t16076, t16433, t16502, t16506, t16534, t3259, t3298, t3322, t342, t43341, t43360, t43378, t4743, t4954, t4976, t4977, t4984, t4996, t4998, t54474, t54909, t55330, t55499, t55586, t73);
        let t55822 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3034(t342, t378, t43400, t11173, t11247, t12094, t12127, t12132, t12133, t12146, t15780, t16393, t16409, t16505, t16506, t16520, t16523, t16574, t16581, t19526, t19569, t3287, t3318, t357, t43350, t4781, t4975, t4981, t4984, t4999, t53670, t53792, t989);
        let t55854 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3035(t1024, t1083, t11804, t11940, t12073, t12143, t12146, t12163, t15670, t15717, t15957, t16390, t16405, t16502, t1651, t16515, t16537, t16540, t3204, t3278, t3287, t3291, t43432, t43450, t43504, t43528, t4757, t4977, t5004, t53865);
        let (t55880, t55894) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3036(t1086, t15886, t3151, t4930, t16543, t3057, t1087, t1089, t1090, t12052, t12122, t12150, t15609, t16432, t1689, t3133, t3259, t3287, t3299, t3304, t3316, t342, t43341, t43413, t43438, t43456, t4999, t54064, t54276, t54365, t54370, t55165, t55550);
        let t55926 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3037(t12077, t1647, t1024, t11202, t11782, t11788, t12080, t12089, t12097, t12154, t15655, t16396, t16399, t16449, t16465, t16468, t16584, t19579, t3075, t3223, t3278, t3283, t3292, t43446, t43456, t4781, t4970, t4975, t5009, t54695, t55612);
    (t55586, t55607, t55643, t55676, t55711, t55746, t55783, t55822, t55854, t55880, t55894, t55926)
}
