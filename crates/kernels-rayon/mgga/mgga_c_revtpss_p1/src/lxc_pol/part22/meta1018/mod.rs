//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1018 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3521;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3522;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3523;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3524;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3525;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3526;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3527;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3528;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3529;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3530;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1018(t1011: f64, t140: f64, t19916: f64, t1668: f64, t372: f64, t4823: f64, t1043: f64, t11249: f64, t11866: f64, t19976: f64, t19907: f64, t3241: f64, t11250: f64, t16027: f64, t16103: f64, t16104: f64, t16228: f64, t19722: f64, t19986: f64, t3095: f64, t43069: f64, t53402: f64, t54490: f64, t54497: f64, t54500: f64, t54521: f64, t54533: f64, t54801: f64, t54811: f64, t55046: f64, t55141: f64, t66187: f64, t6288: f64, t697: f64, t11710: f64, t19872: f64, t3091: f64, t19979: f64, t3153: f64, t19968: f64, t3111: f64, t15850: f64, t4817: f64, t11672: f64, t11883: f64, t15725: f64, t15830: f64, t16226: f64, t1675: f64, t19873: f64, t20083: f64, t42215: f64, t4831: f64, t54546: f64, t54550: f64, t54553: f64, t55356: f64, t6289: f64, t66128: f64, t11921: f64, t19399: f64, t247: f64, t4837: f64, t15752: f64, t19741: f64, t43240: f64, t6267: f64, t16088: f64, t380: f64, t4746: f64, t4866: f64, t4900: f64, t1065: f64, t6299: f64, t3105: f64, t6317: f64, t1068: f64, t15689: f64, t15692: f64, t15907: f64, t16067: f64, t16089: f64, t16128: f64, t16229: f64, t19705: f64, t19819: f64, t19831: f64, t3092: f64, t3116: f64, t3117: f64, t43297: f64, t4772: f64, t54599: f64, t54899: f64, t606: f64, t64912: f64, t15794: f64, t15926: f64, t15993: f64, t18937: f64, t11875: f64, t15785: f64, t15906: f64, t16081: f64, t19450: f64, t19639: f64, t20089: f64, t42571: f64, t43279: f64, t4912: f64, t4915: f64, t53586: f64, t54623: f64, t54638: f64, t54646: f64, t54648: f64, t54916: f64, t6263: f64, t6271: f64, t63297: f64, t127: f64, t15700: f64, t19981: f64, t1045: f64, t11774: f64, t11927: f64, t15591: f64, t15696: f64, t15917: f64, t16043: f64, t19611: f64, t19620: f64, t19626: f64, t19836: f64, t19861: f64, t19982: f64, t3115: f64, t43066: f64, t4919: f64, t53923: f64, t54651: f64, t54656: f64, t63253: f64, t63364: f64, t64848: f64, t65192: f64, t11852: f64, t15140: f64, t15614: f64, t42328: f64, t42907: f64, t43082: f64, t53914: f64, t54667: f64, t54678: f64, t54680: f64, t54687: f64, t54693: f64, t54704: f64, t54708: f64, t11675: f64, t11703: f64, t11933: f64, t15585: f64, t19501: f64, t19636: f64, t19641: f64, t19731: f64, t19838: f64, t42216: f64, t42675: f64, t42765: f64, t42926: f64, t42929: f64, t42932: f64, t43139: f64, t43244: f64, t4579: f64, t4892: f64, t54733: f64, t55209: f64, t6268: f64, t905: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t66686, t66689, t66702, t66712, t66714) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3521(t1011, t140, t19916, t1668, t372, t4823, t1043, t11249, t11866, t19976, t19907, t3241);
        let t66716 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3522(t11250, t16027, t16103, t16104, t16228, t19722, t19986, t3095, t43069, t53402, t54490, t54497, t54500, t54521, t54533, t54801, t54811, t55046, t55141, t66187, t66686, t66689, t66702, t66712, t66714);
        let (t66721, t66731, t66734, t66739, t66747) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3523(t1011, t6288, t697, t11710, t19872, t3091, t19979, t3153, t372, t19968, t3111, t15850, t4817);
        let t66749 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3524(t11672, t11883, t15725, t15830, t16226, t1675, t19873, t20083, t42215, t4831, t54546, t54550, t54553, t55356, t6289, t66128, t66721, t66731, t66734, t66739, t66747);
        let (t66752, t66758, t66763, t66766) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3525(t11921, t19399, t247, t4837, t15752, t19741, t3091, t43240, t6267, t16088, t380, t4746);
        let (t66771, t66777, t66793) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3526(t4866, t4900, t1065, t372, t6299, t3105, t6317, t1068, t15689, t15692, t15907, t16067, t16089, t16128, t16226, t16229, t19705, t19819, t19831, t247, t3092, t3116, t3117, t43297, t4772, t4837, t54599, t54899, t606, t64912, t66752, t66758, t66763, t66766);
        let t66827 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3527(t15794, t15926, t1011, t15993, t18937, t11875, t15785, t15906, t16081, t19450, t19639, t20089, t3117, t42571, t43279, t4912, t4915, t53586, t54623, t54638, t54646, t54648, t54916, t6263, t6271, t63297);
        let t66865 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3528(t127, t15700, t19979, t19981, t1011, t1045, t11774, t11927, t15591, t15696, t15907, t15917, t16043, t16081, t19611, t19620, t19626, t19836, t19861, t19982, t3115, t3117, t43066, t4915, t4919, t53923, t54651, t54656, t63253, t63364, t64848, t65192);
        let t66893 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3529(t1045, t11774, t11852, t15140, t15614, t15692, t15696, t15700, t16229, t1668, t19986, t372, t42328, t42907, t43082, t53914, t54667, t54678, t54680, t54687, t54693, t54704, t54708, t66689);
        let t66925 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3530(t11675, t11703, t11933, t15585, t15689, t19501, t19636, t19641, t19731, t19838, t3092, t372, t42216, t42675, t42765, t42926, t42929, t42932, t43069, t43139, t43244, t4579, t4823, t4892, t4900, t54733, t55209, t6268, t905);
    (t66716, t66734, t66749, t66766, t66771, t66777, t66793, t66827, t66865, t66893, t66925)
}
