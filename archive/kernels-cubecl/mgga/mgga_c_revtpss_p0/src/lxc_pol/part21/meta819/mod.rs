//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta819 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3021;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3022;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3023;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3024;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3025;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3026;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta819<F: Float>(t3133: F, t4772: F, t3105: F, t4797: F, t15725: F, t15827: F, t11921: F, t16152: F, t247: F, t4837: F, t1045: F, t1068: F, t11859: F, t15728: F, t15839: F, t15895: F, t15899: F, t16154: F, t1675: F, t3115: F, t3117: F, t3155: F, t42643: F, t42675: F, t42830: F, t43091: F, t43121: F, t4907: F, t53792: F, t53310: F, t53351: F, t53377: F, t53395: F, t53425: F, t53455: F, t53490: F, t53528: F, t53549: F, t53581: F, t53617: F, t53645: F, t53682: F, t53716: F, t53759: F, t53785: F, t53816: F, t53844: F, t53883: F, t53920: F, t53954: F, t53987: F, t54013: F, t54049: F, t54083: F, t54110: F, t54149: F, t54176: F, t54195: F, t54224: F, t54275: F, t54308: F, t54346: F, t54389: F, t54418: F, t54455: F, t54495: F, t54526: F, t54559: F, t54589: F, t54622: F, t54653: F, t54684: F, t54712: F, t54735: F, t54770: F, t54806: F, t54843: F, t54880: F, t54904: F, t54945: F, t54977: F, t55016: F, t55039: F, t55069: F, t55096: F, t55140: F, t55163: F, t55198: F, t55237: F, t55271: F, t55303: F, t55338: F, t1000: F, t1076: F, t1079: F, t11122: F, t11123: F, t11195: F, t11201: F, t11202: F, t11207: F, t12040: F, t12174: F, t16287: F, t16305: F, t16328: F, t16362: F, t16374: F, t1695: F, t225: F, t3047: F, t3060: F, t3067: F, t3076: F, t3269: F, t3271: F, t3325: F, t342: F, t385: F, t42060: F, t42067: F, t4747: F, t4752: F, t4778: F, t4935: F, t4947: F, t5015: F, t53223: F, t53273: F, t53281: F, t54983: F, t995: F, t996: F, t3057: F, t4930: F, t15886: F, t378: F, t3046: F, t1097: F, t11178: F, t11190: F, t11224: F, t16255: F, t16275: F, t16292: F, t16302: F, t16340: F, t16371: F, t1652: F, t16603: F, t3052: F, t3058: F, t3059: F, t3063: F, t3066: F, t3268: F, t3326: F, t43637: F, t43670: F, t4773: F, t54955: F, t1072: F, t1647: F, t3259: F, t1071: F, t15669: F, t15654: F, t12050: F, t15907: F, t16076: F, t3153: F, t1024: F, t1043: F, t1082: F, t1087: F, t1089: F, t11173: F, t11940: F, t12097: F, t12122: F, t12127: F, t16237: F, t16432: F, t16458: F, t16461: F, t16559: F, t16566: F, t3223: F, t43443: F, t43598: F, t4983: F, t4992: F, t4998: F, t5004: F, t53089: F, t53516: F, t53909: F, t54130: F, t15780: F, t3302: F, t11788: F, t12073: F, t12149: F, t15604: F, t15837: F, t16433: F, t16436: F, t16440: F, t16449: F, t16482: F, t3204: F, t3278: F, t354: F, t43432: F, t43453: F, t43528: F, t4781: F, t54360: F, t54931: F, t54936: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t55345, t55371) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3021::<F>(t3133, t4772, t3105, t4797, t15725, t15827, t11921, t16152, t247, t4837, t1045, t1068, t11859, t15728, t15839, t15895, t15899, t16154, t1675, t3115, t3117, t3155, t42643, t42675, t42830, t43091, t43121, t4907, t53792);
        let t55377 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3022::<F>(t53310, t53351, t53377, t53395, t53425, t53455, t53490, t53528, t53549, t53581, t53617, t53645, t53682, t53716, t53759, t53785, t53816, t53844, t53883, t53920, t53954, t53987, t54013, t54049, t54083, t54110, t54149, t54176, t54195, t54224, t54275, t54308, t54346, t54389, t54418, t54455, t54495, t54526, t54559, t54589, t54622, t54653, t54684, t54712, t54735, t54770, t54806, t54843, t54880, t54904, t54945, t54977, t55016, t55039, t55069, t55096, t55140, t55163, t55198, t55237, t55271, t55303, t55338, t55371);
        let t55405 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3023::<F>(t1000, t1076, t1079, t11122, t11123, t11195, t11201, t11202, t11207, t12040, t12174, t16287, t16305, t16328, t16362, t16374, t1695, t225, t3047, t3060, t3067, t3076, t3269, t3271, t3325, t342, t385, t42060, t42067, t4747, t4752, t4778, t4935, t4947, t5015, t53223, t53273, t53281, t54983, t55377, t995, t996);
        let t55453 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3024::<F>(t3057, t4930, t15886, t378, t3046, t1000, t1079, t1097, t11123, t11178, t11190, t11224, t12040, t12174, t16255, t16275, t16287, t16292, t16302, t16340, t16371, t1652, t16603, t3052, t3058, t3059, t3060, t3063, t3066, t3076, t3268, t3271, t3326, t43637, t43670, t4747, t4752, t4773, t4935, t5015, t54955, t996);
        let (t55458, t55461, t55464, t55475, t55499, t55524) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3025::<F>(t1072, t3057, t1647, t3259, t1071, t15669, t15654, t12050, t15907, t16076, t3153, t1024, t1043, t1082, t1087, t1089, t11173, t11940, t12097, t12122, t12127, t16237, t16432, t16458, t16461, t16559, t16566, t3223, t43443, t43598, t4983, t4992, t4998, t5004, t53089, t53516, t53909, t54130);
        let (t55550, t55562) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3026::<F>(t15780, t3302, t1024, t1043, t1082, t1089, t11788, t12073, t12122, t12127, t12149, t15604, t15837, t16432, t16433, t16436, t16440, t16449, t16482, t3059, t3204, t3278, t354, t43432, t43453, t43528, t4772, t4781, t53273, t54360, t54931, t54936);
    (t55345, t55377, t55405, t55453, t55458, t55461, t55464, t55475, t55499, t55524, t55550, t55562)
}
