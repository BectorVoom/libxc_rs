//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1010 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3463;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3464;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3465;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3466;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3467;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3468;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1010<F: Float>(t12166: F, t1678: F, t342: F, t12077: F, t11782: F, t12050: F, t15655: F, t16390: F, t16410: F, t16446: F, t16468: F, t16479: F, t16499: F, t16502: F, t16506: F, t16544: F, t16555: F, t16562: F, t16566: F, t19399: F, t19443: F, t19450: F, t19457: F, t19573: F, t19576: F, t3133: F, t3204: F, t3223: F, t3291: F, t3316: F, t357: F, t42261: F, t4857: F, t4930: F, t4967: F, t4999: F, t53877: F, t6368: F, t3151: F, t6244: F, t1024: F, t12132: F, t12146: F, t12154: F, t15648: F, t16449: F, t16515: F, t16534: F, t19492: F, t19498: F, t19549: F, t19569: F, t19572: F, t20146: F, t3043: F, t3304: F, t3318: F, t43378: F, t43438: F, t43450: F, t43456: F, t4757: F, t4954: F, t4981: F, t5004: F, t5005: F, t55579: F, t55583: F, t6365: F, t6371: F, t6389: F, t20050: F, t3106: F, t1063: F, t247: F, t42447: F, t6092: F, t11921: F, t15716: F, t19456: F, t11656: F, t15728: F, t15834: F, t15850: F, t16190: F, t16205: F, t19677: F, t19819: F, t19944: F, t3116: F, t4808: F, t4834: F, t4837: F, t4869: F, t54982: F, t54988: F, t64647: F, t64772: F, t64831: F, t3140: F, t6235: F, t3149: F, t19696: F, t3168: F, t15830: F, t4817: F, t1042: F, t1047: F, t15707: F, t15811: F, t15952: F, t16210: F, t1675: F, t19649: F, t19697: F, t2853: F, t2862: F, t3136: F, t3157: F, t3181: F, t42939: F, t4875: F, t53692: F, t54838: F, t6308: F, t64835: F, t11986: F, t6100: F, t20054: F, t3075: F, t5819: F, t2251: F, t5825: F, t19701: F, t3127: F, t3172: F, t63212: F, t63214: F, t63216: F, t63218: F, t63220: F, t63222: F, t63224: F, t63226: F, t63228: F, t63579: F, t63581: F, t63583: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t65239 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3463::<F>(t12166, t1678, t342, t12077, t11782, t12050, t15655, t16390, t16410, t16446, t16468, t16479, t16499, t16502, t16506, t16544, t16555, t16562, t16566, t19399, t19443, t19450, t19457, t19573, t19576, t3133, t3204, t3223, t3291, t3316, t357, t42261, t4857, t4930, t4967, t4999, t53877, t6368);
        let (t65261, t65279) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3464::<F>(t3151, t6244, t1024, t11782, t12132, t12146, t12154, t15648, t15655, t16449, t16515, t16534, t19492, t19498, t19549, t19569, t19572, t20146, t3043, t3204, t3304, t3318, t43378, t43438, t43450, t43456, t4757, t4954, t4981, t5004, t5005, t55579, t55583, t6365, t6371, t6389);
        let t65316 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3465::<F>(t20050, t3106, t1063, t247, t42447, t6092, t11921, t15716, t19456, t11656, t15728, t15834, t15850, t16190, t16205, t19677, t19819, t19944, t3116, t4808, t4834, t4837, t4869, t54982, t54988, t64647, t64772, t64831);
        let (t65338, t65353) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3466::<F>(t3140, t6235, t3149, t19696, t3168, t15830, t4817, t1042, t1047, t15707, t15811, t15952, t16210, t1675, t19649, t19697, t247, t2853, t2862, t3116, t3136, t3157, t3181, t42939, t4834, t4837, t4875, t53692, t54838, t6244, t6308, t64835);
        let (t65357, t65359, t65365, t65370) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3467::<F>(t1063, t11986, t247, t6100, t20054, t3106, t3075, t5819, t2251, t5825);
        let (t65376, t65388) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3468::<F>(t19701, t3127, t3172, t63212, t63214, t63216, t63218, t63220, t63222, t63224, t63226, t63228, t63579, t63581, t63583);
    (t65239, t65261, t65279, t65316, t65338, t65353, t65357, t65359, t65365, t65370, t65376, t65388)
}
