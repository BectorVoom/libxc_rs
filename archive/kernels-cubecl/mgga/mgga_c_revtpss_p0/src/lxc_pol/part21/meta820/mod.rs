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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta820<F: Float>(t342: F, t378: F, t43536: F, t11631: F, t43350: F, t16558: F, t989: F, t1071: F, t12166: F, t12077: F, t11247: F, t1678: F, t43346: F, t42872: F, t12046: F, t1647: F, t1082: F, t12053: F, t12078: F, t12079: F, t12116: F, t16427: F, t16443: F, t16446: F, t16520: F, t16555: F, t16562: F, t3223: F, t42359: F, t43154: F, t4961: F, t53670: F, t54983: F, t15893: F, t3153: F, t16551: F, t1043: F, t1089: F, t12097: F, t12149: F, t15957: F, t16152: F, t16410: F, t16479: F, t16523: F, t16534: F, t16577: F, t19502: F, t3043: F, t3287: F, t43438: F, t43450: F, t43520: F, t43524: F, t4964: F, t4976: F, t4988: F, t5012: F, t53340: F, t53506: F, t54026: F, t55499: F, t12153: F, t4746: F, t16237: F, t359: F, t1024: F, t12119: F, t12143: F, t12146: F, t12154: F, t15670: F, t15837: F, t16390: F, t16406: F, t16499: F, t16544: F, t3204: F, t3288: F, t3291: F, t380: F, t42261: F, t43357: F, t54955: F, t55377: F, t999: F, t15654: F, t3286: F, t16543: F, t3046: F, t1087: F, t12133: F, t12160: F, t15780: F, t16183: F, t16393: F, t16468: F, t16488: F, t16537: F, t16573: F, t16581: F, t19603: F, t43360: F, t4980: F, t4984: F, t4995: F, t4996: F, t4999: F, t55345: F, t12066: F, t12094: F, t12122: F, t12128: F, t12168: F, t16381: F, t16540: F, t16578: F, t19608: F, t3259: F, t3304: F, t3309: F, t43443: F, t43453: F, t43562: F, t43598: F, t4866: F, t4893: F, t4977: F, t4981: F, t53792: F, t54276: F, t15669: F, t1651: F, t11804: F, t12057: F, t12150: F, t12157: F, t12167: F, t16076: F, t16433: F, t16502: F, t16506: F, t3298: F, t3322: F, t43341: F, t43378: F, t4743: F, t4954: F, t4998: F, t54474: F, t54909: F, t55330: F, t73: F, t43400: F, t11173: F, t12127: F, t12132: F, t16409: F, t16505: F, t16574: F, t19526: F, t19569: F, t3318: F, t357: F, t4781: F, t4975: F, t1083: F, t11940: F, t12073: F, t12163: F, t15717: F, t16405: F, t16515: F, t3278: F, t43432: F, t43504: F, t43528: F, t4757: F, t5004: F, t53865: F, t1086: F, t15886: F, t3151: F, t4930: F, t3057: F, t1090: F, t12052: F, t15609: F, t16432: F, t1689: F, t3133: F, t3299: F, t3316: F, t43413: F, t43456: F, t54064: F, t54365: F, t54370: F, t55165: F, t55550: F, t11202: F, t11782: F, t11788: F, t12080: F, t12089: F, t15655: F, t16396: F, t16399: F, t16449: F, t16465: F, t16584: F, t19579: F, t3075: F, t3283: F, t3292: F, t43446: F, t4970: F, t5009: F, t54695: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t55569, t55570, t55575, t55579, t55583, t55586) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3027::<F>(t342, t378, t43536, t11631, t43350, t16558, t989, t1071, t12166, t12077, t11247, t1678);
        let t55607 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3028::<F>(t342, t378, t43346, t42872, t43350, t12046, t1647, t1082, t11247, t12053, t12078, t12079, t12116, t16427, t16443, t16446, t16520, t16555, t16562, t3223, t42359, t43154, t4961, t53670, t54983, t55569, t55570, t55575, t55579, t55583, t55586);
        let (t55612, t55643) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3029::<F>(t15893, t3153, t16551, t989, t1043, t1089, t12097, t12149, t15957, t16152, t16410, t16443, t16479, t16523, t16534, t16555, t16577, t19502, t3043, t3223, t3287, t43438, t43450, t43520, t43524, t4964, t4976, t4988, t5012, t53340, t53506, t54026, t55499);
        let t55676 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3030::<F>(t12153, t4746, t16237, t359, t1024, t1082, t12119, t12143, t12146, t12154, t15670, t15837, t16390, t16406, t16499, t16544, t3204, t3288, t3291, t342, t380, t42261, t43357, t4964, t54955, t55377, t999);
        let t55711 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3031::<F>(t15654, t3286, t16543, t3046, t1071, t1087, t1089, t12133, t12146, t12154, t12160, t15780, t16183, t16393, t16410, t16468, t16488, t16537, t16573, t16581, t19603, t3043, t3287, t3288, t43360, t4980, t4984, t4995, t4996, t4999, t55345);
        let t55746 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3032::<F>(t4746, t4995, t1087, t1089, t12066, t12079, t12094, t12122, t12128, t12168, t16381, t1647, t16540, t16578, t19608, t3259, t3304, t3309, t43357, t43443, t43453, t43520, t43524, t43562, t43598, t4866, t4893, t4977, t4981, t53792, t54276);
        let t55783 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3033::<F>(t15669, t3286, t1651, t378, t11804, t12057, t12149, t12150, t12157, t12167, t12168, t16076, t16433, t16502, t16506, t16534, t3259, t3298, t3322, t342, t43341, t43360, t43378, t4743, t4954, t4976, t4977, t4984, t4996, t4998, t54474, t54909, t55330, t55499, t55586, t73);
        let t55822 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3034::<F>(t342, t378, t43400, t11173, t11247, t12094, t12127, t12132, t12133, t12146, t15780, t16393, t16409, t16505, t16506, t16520, t16523, t16574, t16581, t19526, t19569, t3287, t3318, t357, t43350, t4781, t4975, t4981, t4984, t4999, t53670, t53792, t989);
        let t55854 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3035::<F>(t1024, t1083, t11804, t11940, t12073, t12143, t12146, t12163, t15670, t15717, t15957, t16390, t16405, t16502, t1651, t16515, t16537, t16540, t3204, t3278, t3287, t3291, t43432, t43450, t43504, t43528, t4757, t4977, t5004, t53865);
        let (t55880, t55894) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3036::<F>(t1086, t15886, t3151, t4930, t16543, t3057, t1087, t1089, t1090, t12052, t12122, t12150, t15609, t16432, t1689, t3133, t3259, t3287, t3299, t3304, t3316, t342, t43341, t43413, t43438, t43456, t4999, t54064, t54276, t54365, t54370, t55165, t55550);
        let t55926 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3037::<F>(t12077, t1647, t1024, t11202, t11782, t11788, t12080, t12089, t12097, t12154, t15655, t16396, t16399, t16449, t16465, t16468, t16584, t19579, t3075, t3223, t3278, t3283, t3292, t43446, t43456, t4781, t4970, t4975, t5009, t54695, t55612);
    (t55586, t55607, t55643, t55676, t55711, t55746, t55783, t55822, t55854, t55880, t55894, t55926)
}
