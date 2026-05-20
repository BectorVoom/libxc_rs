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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1018<F: Float>(t1011: F, t140: F, t19916: F, t1668: F, t372: F, t4823: F, t1043: F, t11249: F, t11866: F, t19976: F, t19907: F, t3241: F, t11250: F, t16027: F, t16103: F, t16104: F, t16228: F, t19722: F, t19986: F, t3095: F, t43069: F, t53402: F, t54490: F, t54497: F, t54500: F, t54521: F, t54533: F, t54801: F, t54811: F, t55046: F, t55141: F, t66187: F, t6288: F, t697: F, t11710: F, t19872: F, t3091: F, t19979: F, t3153: F, t19968: F, t3111: F, t15850: F, t4817: F, t11672: F, t11883: F, t15725: F, t15830: F, t16226: F, t1675: F, t19873: F, t20083: F, t42215: F, t4831: F, t54546: F, t54550: F, t54553: F, t55356: F, t6289: F, t66128: F, t11921: F, t19399: F, t247: F, t4837: F, t15752: F, t19741: F, t43240: F, t6267: F, t16088: F, t380: F, t4746: F, t4866: F, t4900: F, t1065: F, t6299: F, t3105: F, t6317: F, t1068: F, t15689: F, t15692: F, t15907: F, t16067: F, t16089: F, t16128: F, t16229: F, t19705: F, t19819: F, t19831: F, t3092: F, t3116: F, t3117: F, t43297: F, t4772: F, t54599: F, t54899: F, t606: F, t64912: F, t15794: F, t15926: F, t15993: F, t18937: F, t11875: F, t15785: F, t15906: F, t16081: F, t19450: F, t19639: F, t20089: F, t42571: F, t43279: F, t4912: F, t4915: F, t53586: F, t54623: F, t54638: F, t54646: F, t54648: F, t54916: F, t6263: F, t6271: F, t63297: F, t127: F, t15700: F, t19981: F, t1045: F, t11774: F, t11927: F, t15591: F, t15696: F, t15917: F, t16043: F, t19611: F, t19620: F, t19626: F, t19836: F, t19861: F, t19982: F, t3115: F, t43066: F, t4919: F, t53923: F, t54651: F, t54656: F, t63253: F, t63364: F, t64848: F, t65192: F, t11852: F, t15140: F, t15614: F, t42328: F, t42907: F, t43082: F, t53914: F, t54667: F, t54678: F, t54680: F, t54687: F, t54693: F, t54704: F, t54708: F, t11675: F, t11703: F, t11933: F, t15585: F, t19501: F, t19636: F, t19641: F, t19731: F, t19838: F, t42216: F, t42675: F, t42765: F, t42926: F, t42929: F, t42932: F, t43139: F, t43244: F, t4579: F, t4892: F, t54733: F, t55209: F, t6268: F, t905: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t66686, t66689, t66702, t66712, t66714) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3521::<F>(t1011, t140, t19916, t1668, t372, t4823, t1043, t11249, t11866, t19976, t19907, t3241);
        let t66716 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3522::<F>(t11250, t16027, t16103, t16104, t16228, t19722, t19986, t3095, t43069, t53402, t54490, t54497, t54500, t54521, t54533, t54801, t54811, t55046, t55141, t66187, t66686, t66689, t66702, t66712, t66714);
        let (t66721, t66731, t66734, t66739, t66747) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3523::<F>(t1011, t6288, t697, t11710, t19872, t3091, t19979, t3153, t372, t19968, t3111, t15850, t4817);
        let t66749 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3524::<F>(t11672, t11883, t15725, t15830, t16226, t1675, t19873, t20083, t42215, t4831, t54546, t54550, t54553, t55356, t6289, t66128, t66721, t66731, t66734, t66739, t66747);
        let (t66752, t66758, t66763, t66766) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3525::<F>(t11921, t19399, t247, t4837, t15752, t19741, t3091, t43240, t6267, t16088, t380, t4746);
        let (t66771, t66777, t66793) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3526::<F>(t4866, t4900, t1065, t372, t6299, t3105, t6317, t1068, t15689, t15692, t15907, t16067, t16089, t16128, t16226, t16229, t19705, t19819, t19831, t247, t3092, t3116, t3117, t43297, t4772, t4837, t54599, t54899, t606, t64912, t66752, t66758, t66763, t66766);
        let t66827 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3527::<F>(t15794, t15926, t1011, t15993, t18937, t11875, t15785, t15906, t16081, t19450, t19639, t20089, t3117, t42571, t43279, t4912, t4915, t53586, t54623, t54638, t54646, t54648, t54916, t6263, t6271, t63297);
        let t66865 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3528::<F>(t127, t15700, t19979, t19981, t1011, t1045, t11774, t11927, t15591, t15696, t15907, t15917, t16043, t16081, t19611, t19620, t19626, t19836, t19861, t19982, t3115, t3117, t43066, t4915, t4919, t53923, t54651, t54656, t63253, t63364, t64848, t65192);
        let t66893 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3529::<F>(t1045, t11774, t11852, t15140, t15614, t15692, t15696, t15700, t16229, t1668, t19986, t372, t42328, t42907, t43082, t53914, t54667, t54678, t54680, t54687, t54693, t54704, t54708, t66689);
        let t66925 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3530::<F>(t11675, t11703, t11933, t15585, t15689, t19501, t19636, t19641, t19731, t19838, t3092, t372, t42216, t42675, t42765, t42926, t42929, t42932, t43069, t43139, t43244, t4579, t4823, t4892, t4900, t54733, t55209, t6268, t905);
    (t66716, t66734, t66749, t66766, t66771, t66777, t66793, t66827, t66865, t66893, t66925)
}
