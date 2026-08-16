//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1010 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3463;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3464;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3465;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3466;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3467;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3468;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1010(t12166: f64, t1678: f64, t342: f64, t12077: f64, t11782: f64, t12050: f64, t15655: f64, t16390: f64, t16410: f64, t16446: f64, t16468: f64, t16479: f64, t16499: f64, t16502: f64, t16506: f64, t16544: f64, t16555: f64, t16562: f64, t16566: f64, t19399: f64, t19443: f64, t19450: f64, t19457: f64, t19573: f64, t19576: f64, t3133: f64, t3204: f64, t3223: f64, t3291: f64, t3316: f64, t357: f64, t42261: f64, t4857: f64, t4930: f64, t4967: f64, t4999: f64, t53877: f64, t6368: f64, t3151: f64, t6244: f64, t1024: f64, t12132: f64, t12146: f64, t12154: f64, t15648: f64, t16449: f64, t16515: f64, t16534: f64, t19492: f64, t19498: f64, t19549: f64, t19569: f64, t19572: f64, t20146: f64, t3043: f64, t3304: f64, t3318: f64, t43378: f64, t43438: f64, t43450: f64, t43456: f64, t4757: f64, t4954: f64, t4981: f64, t5004: f64, t5005: f64, t55579: f64, t55583: f64, t6365: f64, t6371: f64, t6389: f64, t20050: f64, t3106: f64, t1063: f64, t247: f64, t42447: f64, t6092: f64, t11921: f64, t15716: f64, t19456: f64, t11656: f64, t15728: f64, t15834: f64, t15850: f64, t16190: f64, t16205: f64, t19677: f64, t19819: f64, t19944: f64, t3116: f64, t4808: f64, t4834: f64, t4837: f64, t4869: f64, t54982: f64, t54988: f64, t64647: f64, t64772: f64, t64831: f64, t3140: f64, t6235: f64, t3149: f64, t19696: f64, t3168: f64, t15830: f64, t4817: f64, t1042: f64, t1047: f64, t15707: f64, t15811: f64, t15952: f64, t16210: f64, t1675: f64, t19649: f64, t19697: f64, t2853: f64, t2862: f64, t3136: f64, t3157: f64, t3181: f64, t42939: f64, t4875: f64, t53692: f64, t54838: f64, t6308: f64, t64835: f64, t11986: f64, t6100: f64, t20054: f64, t3075: f64, t5819: f64, t2251: f64, t5825: f64, t19701: f64, t3127: f64, t3172: f64, t63212: f64, t63214: f64, t63216: f64, t63218: f64, t63220: f64, t63222: f64, t63224: f64, t63226: f64, t63228: f64, t63579: f64, t63581: f64, t63583: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t65239 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3463(t12166, t1678, t342, t12077, t11782, t12050, t15655, t16390, t16410, t16446, t16468, t16479, t16499, t16502, t16506, t16544, t16555, t16562, t16566, t19399, t19443, t19450, t19457, t19573, t19576, t3133, t3204, t3223, t3291, t3316, t357, t42261, t4857, t4930, t4967, t4999, t53877, t6368);
        let (t65261, t65279) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3464(t3151, t6244, t1024, t11782, t12132, t12146, t12154, t15648, t15655, t16449, t16515, t16534, t19492, t19498, t19549, t19569, t19572, t20146, t3043, t3204, t3304, t3318, t43378, t43438, t43450, t43456, t4757, t4954, t4981, t5004, t5005, t55579, t55583, t6365, t6371, t6389);
        let t65316 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3465(t20050, t3106, t1063, t247, t42447, t6092, t11921, t15716, t19456, t11656, t15728, t15834, t15850, t16190, t16205, t19677, t19819, t19944, t3116, t4808, t4834, t4837, t4869, t54982, t54988, t64647, t64772, t64831);
        let (t65338, t65353) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3466(t3140, t6235, t3149, t19696, t3168, t15830, t4817, t1042, t1047, t15707, t15811, t15952, t16210, t1675, t19649, t19697, t247, t2853, t2862, t3116, t3136, t3157, t3181, t42939, t4834, t4837, t4875, t53692, t54838, t6244, t6308, t64835);
        let (t65357, t65359, t65365, t65370) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3467(t1063, t11986, t247, t6100, t20054, t3106, t3075, t5819, t2251, t5825);
        let (t65376, t65388) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3468(t19701, t3127, t3172, t63212, t63214, t63216, t63218, t63220, t63222, t63224, t63226, t63228, t63579, t63581, t63583);
    (t65239, t65261, t65279, t65316, t65338, t65353, t65357, t65359, t65365, t65370, t65376, t65388)
}
