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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1013(t19894: f64, t3127: f64, t3172: f64, t15707: f64, t15734: f64, t19882: f64, t3188: f64, t16190: f64, t4820: f64, t1063: f64, t19662: f64, t19667: f64, t1042: f64, t11714: f64, t1592: f64, t42665: f64, t42672: f64, t4825: f64, t53290: f64, t53293: f64, t53926: f64, t54419: f64, t6308: f64, t6312: f64, t6331: f64, t11994: f64, t19920: f64, t4866: f64, t373: f64, t19692: f64, t19650: f64, t4837: f64, t15697: f64, t15728: f64, t1671: f64, t19651: f64, t3150: f64, t3155: f64, t53298: f64, t53300: f64, t53302: f64, t53308: f64, t53317: f64, t53326: f64, t55141: f64, t55195: f64, t19929: f64, t19933: f64, t19676: f64, t12021: f64, t15193: f64, t15817: f64, t15970: f64, t15975: f64, t16138: f64, t19688: f64, t19738: f64, t19741: f64, t19792: f64, t19800: f64, t3124: f64, t4583: f64, t4801: f64, t4823: f64, t4869: f64, t6302: f64, t65433: f64, t16158: f64, t4834: f64, t19791: f64, t11977: f64, t15839: f64, t15847: f64, t15850: f64, t16149: f64, t1675: f64, t19649: f64, t19878: f64, t19930: f64, t19940: f64, t2858: f64, t4831: f64, t4875: f64, t53353: f64, t54137: f64, t19781: f64, t3091: f64, t43131: f64, t19939: f64, t11262: f64, t3161: f64, t6311: f64, t11274: f64, t20029: f64, t11656: f64, t16140: f64, t19895: f64, t43268: f64, t53359: f64, t53363: f64, t53692: f64, t54739: f64, t6262: f64, t15775: f64, t1032: f64, t1040: f64, t19856: f64, t11277: f64, t19826: f64, t1047: f64, t15830: f64, t16167: f64, t16172: f64, t19934: f64, t3106: f64, t3162: f64, t42371: f64, t4808: f64, t16163: f64, t4879: f64, t19681: f64, t11710: f64, t19625: f64, t4899: f64, t19687: f64, t3160: f64, t65338: f64, t11672: f64, t11675: f64, t15963: f64, t19501: f64, t19682: f64, t19702: f64, t19778: f64, t19782: f64, t3092: f64, t3164: f64, t42391: f64, t4783: f64, t54144: f64, t54471: f64, t6263: f64, t15772: f64, t1045: f64, t18281: f64, t19622: f64, t19675: f64, t3059: f64, t3075: f64, t3117: f64, t42121: f64, t42124: f64, t42141: f64, t43291: f64, t43297: f64, t4803: f64, t4872: f64, t53389: f64, t5825: f64, t6271: f64, t999: f64, t1065: f64, t19380: f64, t1062: f64, t19463: f64, t15791: f64, t15938: f64, t16196: f64, t16201: f64, t19668: f64, t19677: f64, t19968: f64, t3101: f64, t3130: f64, t4806: f64, t53393: f64, t60834: f64, t60838: f64, t906: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t65444, t65446, t65454, t65456, t65459, t65462) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3479(t19894, t3127, t3172, t15707, t15734, t19882, t3188, t16190, t4820, t1063, t19662, t19667);
        let t65468 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3480(t1042, t11714, t1592, t3127, t42665, t42672, t4825, t53290, t53293, t53926, t54419, t6308, t6312, t6331, t65444, t65446, t65454, t65456, t65459, t65462);
        let (t65481, t65482, t65497) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3481(t11994, t19920, t4866, t373, t19692, t3127, t3172, t19650, t4837, t1042, t15697, t15728, t1671, t19651, t3150, t3155, t53298, t53300, t53302, t53308, t53317, t53326, t55141, t55195);
        let t65533 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3482(t1063, t19929, t3172, t19933, t19676, t3127, t1042, t11994, t12021, t15193, t15817, t15970, t15975, t16138, t19688, t19738, t19741, t19792, t19800, t3124, t3188, t4583, t4801, t4823, t4869, t6302, t65433);
        let t65563 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3483(t16158, t4834, t19791, t3127, t3172, t1042, t11977, t11994, t15839, t15847, t15850, t16149, t1675, t19649, t19878, t19930, t19940, t2858, t3188, t4831, t4837, t4875, t53353, t53926, t54137, t6302);
        let (t65567, t65570, t65581, t65585, t65589) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3484(t19781, t3091, t43131, t19939, t3127, t3172, t11262, t3161, t6311, t11274, t20029, t11656, t19920);
        let t65591 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3485(t11656, t11994, t15707, t16140, t1671, t19895, t43268, t4825, t53359, t53363, t53692, t54739, t6308, t65567, t65570, t65581, t65585, t65589);
        let (t65596, t65598, t65610, t65613, t65618) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3486(t11262, t3127, t6262, t15817, t4820, t15775, t4834, t1032, t1040, t19856, t11277, t19826);
        let t65626 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3487(t1042, t1047, t11656, t15707, t15830, t16167, t16172, t19792, t19934, t19940, t3106, t3161, t3162, t42371, t4808, t6312, t65482, t65596, t65598, t65610, t65613, t65618);
        let t65659 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3488(t16163, t4879, t1063, t19681, t3172, t11710, t19625, t4899, t19687, t3160, t65338, t11672, t11675, t11994, t15963, t1671, t19501, t19682, t19702, t19778, t19782, t3092, t3164, t3188, t42391, t4783, t54144, t54471, t6263);
        let t65693 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3489(t15772, t4834, t1042, t1045, t15830, t15850, t18281, t19622, t19675, t19682, t2858, t3059, t3075, t3106, t3117, t3127, t42121, t42124, t42141, t43291, t43297, t4803, t4872, t53389, t5825, t6271, t999);
        let t65727 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3490(t1065, t19380, t1062, t19463, t1042, t1063, t11994, t15791, t15938, t16196, t16201, t19668, t19677, t19930, t19968, t3101, t3106, t3127, t3130, t4806, t4834, t53393, t60834, t60838, t906);
    (t65468, t65481, t65497, t65533, t65563, t65591, t65626, t65659, t65693, t65727)
}
