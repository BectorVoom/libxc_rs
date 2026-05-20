//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1013 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3479;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3480;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3481;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3482;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3483;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3484;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3485;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3486;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3487;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3488;
use chunk10::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3489;
use chunk11::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3490;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1013<F: Float>(t19894: F, t3127: F, t3172: F, t15707: F, t15734: F, t19882: F, t3188: F, t16190: F, t4820: F, t1063: F, t19662: F, t19667: F, t1042: F, t11714: F, t1592: F, t42665: F, t42672: F, t4825: F, t53290: F, t53293: F, t53926: F, t54419: F, t6308: F, t6312: F, t6331: F, t11994: F, t19920: F, t4866: F, t373: F, t19692: F, t19650: F, t4837: F, t15697: F, t15728: F, t1671: F, t19651: F, t3150: F, t3155: F, t53298: F, t53300: F, t53302: F, t53308: F, t53317: F, t53326: F, t55141: F, t55195: F, t19929: F, t19933: F, t19676: F, t12021: F, t15193: F, t15817: F, t15970: F, t15975: F, t16138: F, t19688: F, t19738: F, t19741: F, t19792: F, t19800: F, t3124: F, t4583: F, t4801: F, t4823: F, t4869: F, t6302: F, t65433: F, t16158: F, t4834: F, t19791: F, t11977: F, t15839: F, t15847: F, t15850: F, t16149: F, t1675: F, t19649: F, t19878: F, t19930: F, t19940: F, t2858: F, t4831: F, t4875: F, t53353: F, t54137: F, t19781: F, t3091: F, t43131: F, t19939: F, t11262: F, t3161: F, t6311: F, t11274: F, t20029: F, t11656: F, t16140: F, t19895: F, t43268: F, t53359: F, t53363: F, t53692: F, t54739: F, t6262: F, t15775: F, t1032: F, t1040: F, t19856: F, t11277: F, t19826: F, t1047: F, t15830: F, t16167: F, t16172: F, t19934: F, t3106: F, t3162: F, t42371: F, t4808: F, t16163: F, t4879: F, t19681: F, t11710: F, t19625: F, t4899: F, t19687: F, t3160: F, t65338: F, t11672: F, t11675: F, t15963: F, t19501: F, t19682: F, t19702: F, t19778: F, t19782: F, t3092: F, t3164: F, t42391: F, t4783: F, t54144: F, t54471: F, t6263: F, t15772: F, t1045: F, t18281: F, t19622: F, t19675: F, t3059: F, t3075: F, t3117: F, t42121: F, t42124: F, t42141: F, t43291: F, t43297: F, t4803: F, t4872: F, t53389: F, t5825: F, t6271: F, t999: F, t1065: F, t19380: F, t1062: F, t19463: F, t15791: F, t15938: F, t16196: F, t16201: F, t19668: F, t19677: F, t19968: F, t3101: F, t3130: F, t4806: F, t53393: F, t60834: F, t60838: F, t906: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t65444, t65446, t65454, t65456, t65459, t65462) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3479::<F>(t19894, t3127, t3172, t15707, t15734, t19882, t3188, t16190, t4820, t1063, t19662, t19667);
        let t65468 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3480::<F>(t1042, t11714, t1592, t3127, t42665, t42672, t4825, t53290, t53293, t53926, t54419, t6308, t6312, t6331, t65444, t65446, t65454, t65456, t65459, t65462);
        let (t65481, t65482, t65497) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3481::<F>(t11994, t19920, t4866, t373, t19692, t3127, t3172, t19650, t4837, t1042, t15697, t15728, t1671, t19651, t3150, t3155, t53298, t53300, t53302, t53308, t53317, t53326, t55141, t55195);
        let t65533 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3482::<F>(t1063, t19929, t3172, t19933, t19676, t3127, t1042, t11994, t12021, t15193, t15817, t15970, t15975, t16138, t19688, t19738, t19741, t19792, t19800, t3124, t3188, t4583, t4801, t4823, t4869, t6302, t65433);
        let t65563 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3483::<F>(t16158, t4834, t19791, t3127, t3172, t1042, t11977, t11994, t15839, t15847, t15850, t16149, t1675, t19649, t19878, t19930, t19940, t2858, t3188, t4831, t4837, t4875, t53353, t53926, t54137, t6302);
        let (t65567, t65570, t65581, t65585, t65589) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3484::<F>(t19781, t3091, t43131, t19939, t3127, t3172, t11262, t3161, t6311, t11274, t20029, t11656, t19920);
        let t65591 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3485::<F>(t11656, t11994, t15707, t16140, t1671, t19895, t43268, t4825, t53359, t53363, t53692, t54739, t6308, t65567, t65570, t65581, t65585, t65589);
        let (t65596, t65598, t65610, t65613, t65618) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3486::<F>(t11262, t3127, t6262, t15817, t4820, t15775, t4834, t1032, t1040, t19856, t11277, t19826);
        let t65626 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3487::<F>(t1042, t1047, t11656, t15707, t15830, t16167, t16172, t19792, t19934, t19940, t3106, t3161, t3162, t42371, t4808, t6312, t65482, t65596, t65598, t65610, t65613, t65618);
        let t65659 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3488::<F>(t16163, t4879, t1063, t19681, t3172, t11710, t19625, t4899, t19687, t3160, t65338, t11672, t11675, t11994, t15963, t1671, t19501, t19682, t19702, t19778, t19782, t3092, t3164, t3188, t42391, t4783, t54144, t54471, t6263);
        let t65693 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3489::<F>(t15772, t4834, t1042, t1045, t15830, t15850, t18281, t19622, t19675, t19682, t2858, t3059, t3075, t3106, t3117, t3127, t42121, t42124, t42141, t43291, t43297, t4803, t4872, t53389, t5825, t6271, t999);
        let t65727 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3490::<F>(t1065, t19380, t1062, t19463, t1042, t1063, t11994, t15791, t15938, t16196, t16201, t19668, t19677, t19930, t19968, t3101, t3106, t3127, t3130, t4806, t4834, t53393, t60834, t60838, t906);
    (t65468, t65481, t65497, t65533, t65563, t65591, t65626, t65659, t65693, t65727)
}
