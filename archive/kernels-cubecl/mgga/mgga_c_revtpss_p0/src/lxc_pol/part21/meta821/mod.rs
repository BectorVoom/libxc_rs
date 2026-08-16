//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta821 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3038;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3039;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3040;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3041;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3042;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3043;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3044;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3045;
use chunk8::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3046;
use chunk9::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3047;
use chunk10::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3048;
use chunk11::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3049;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta821<F: Float>(t1086: F, t4930: F, t994: F, t342: F, t378: F, t43471: F, t3154: F, t43350: F, t16565: F, t989: F, t1071: F, t12046: F, t3298: F, t4743: F, t1024: F, t11247: F, t11788: F, t11902: F, t12105: F, t15648: F, t15655: F, t16414: F, t16485: F, t16569: F, t1692: F, t3278: F, t3288: F, t3291: F, t3295: F, t3305: F, t3317: F, t3318: F, t53670: F, t53877: F, t55880: F, t3316: F, t19602: F, t19607: F, t1082: F, t1087: F, t1089: F, t1093: F, t11620: F, t12047: F, t12052: F, t12070: F, t12086: F, t12100: F, t12124: F, t12128: F, t15886: F, t16496: F, t16509: F, t16566: F, t16568: F, t1678: F, t3204: F, t3319: F, t381: F, t4857: F, t4954: F, t52977: F, t54112: F, t54479: F, t55586: F, t12166: F, t1647: F, t1043: F, t11782: F, t12032: F, t12074: F, t12137: F, t12169: F, t15717: F, t16450: F, t16552: F, t16554: F, t16559: F, t16561: F, t1668: F, t1685: F, t3223: F, t3302: F, t357: F, t42278: F, t43446: F, t4893: F, t4981: F, t4983: F, t4996: F, t5005: F, t53904: F, t54909: F, t55499: F, t4746: F, t4980: F, t11202: F, t11940: F, t12111: F, t12157: F, t16152: F, t16381: F, t16402: F, t16529: F, t16544: F, t3287: F, t3313: F, t43378: F, t4964: F, t4967: F, t5004: F, t53192: F, t53683: F, t54249: F, t379: F, t1000: F, t1076: F, t1079: F, t1096: F, t1097: F, t11128: F, t11174: F, t11177: F, t11187: F, t11210: F, t11220: F, t12173: F, t15579: F, t16295: F, t16314: F, t16340: F, t1651: F, t16591: F, t16592: F, t1696: F, t3047: F, t3052: F, t3058: F, t3060: F, t3269: F, t3271: F, t41993: F, t42041: F, t4773: F, t4778: F, t4947: F, t5016: F, t53108: F, t55458: F, t55461: F, t55464: F, t55475: F, t55524: F, t55562: F, t55607: F, t55643: F, t55676: F, t55711: F, t55746: F, t55783: F, t55822: F, t55854: F, t55894: F, t55926: F, t995: F, t996: F, t1100: F, t1102: F, t12190: F, t15562: F, t16612: F, t198: F, t3329: F, t3336: F, t336: F, t5023: F, t5024: F, t52762: F, t52806: F, t52808: F, t53011: F, t53056: F, t53107: F, t53163: F, t53217: F, t54238: F, t54240: F, t54242: F, t54245: F, t55405: F, t55453: F, t30: F, t265: F, t393: F, t51814: F, t52167: F, t52197: F, t52227: F, t52870: F, t52883: F, t52906: F, t52924: F, t10326: F, t1106: F, t11095: F, t12201: F, t13312: F, t1468: F, t1469: F, t15083: F, t1587: F, t16618: F, t1704: F, t2257: F, t2258: F, t3340: F, t395: F, t4186: F, t45: F, t4560: F, t49889: F, t5028: F, t51827: F, t51829: F, t51831: F, t51833: F, t51835: F, t605: F, t606: F, t9344: F, dens_threshold: F, rho0: F, zeta_threshold: F, t10356: F, t16714: F, t128: F, t3360: F, t16737: F, t1120: F, t16724: F, t2251: F, t5051: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t55934, t55938, t55939, t55944, t55948) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3038::<F>(t1086, t4930, t994, t342, t378, t43471, t3154, t43350, t16565, t989, t1071, t12046);
        let t55966 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3039::<F>(t3298, t4743, t1024, t11247, t11788, t11902, t12105, t15648, t15655, t16414, t16485, t16569, t1692, t3278, t3288, t3291, t3295, t3305, t3317, t3318, t53670, t53877, t55880, t55934, t55938, t55939, t55944, t55948);
        let t56001 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3040::<F>(t3316, t4743, t19602, t994, t19607, t1082, t1087, t1089, t1093, t11620, t12047, t12052, t12070, t12086, t12100, t12124, t12128, t15886, t16496, t16509, t16566, t16568, t1678, t3204, t3278, t3319, t381, t4857, t4954, t52977, t54112, t54479, t55586);
        let t56041 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3041::<F>(t12166, t1647, t1043, t1087, t1089, t11620, t11782, t12032, t12074, t12137, t12169, t15717, t16450, t16552, t16554, t16559, t16561, t1668, t1685, t3223, t3302, t357, t42278, t43446, t4857, t4893, t4954, t4981, t4983, t4996, t5005, t53904, t54479, t54909, t55499);
        let t56075 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3042::<F>(t4746, t4980, t1082, t1087, t1089, t11202, t11782, t11788, t11940, t12111, t12124, t12157, t16152, t16381, t16402, t16529, t16544, t3204, t3287, t3291, t3313, t378, t43378, t4857, t4964, t4967, t5004, t53192, t53683, t54249, t989);
        let t56099 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3043::<F>(t342, t379, t1000, t1076, t1079, t1096, t1097, t11128, t11174, t11177, t11187, t11210, t11220, t12173, t15579, t15648, t16295, t16314, t16340, t1651, t16591, t16592, t1696, t3047, t3052, t3058, t3060, t3269, t3271, t41993, t42041, t4773, t4778, t4947, t5016, t53108, t54112, t55458, t55461, t55464, t55475, t55524, t55562, t55607, t55643, t55676, t55711, t55746, t55783, t55822, t55854, t55894, t55926, t55966, t56001, t56041, t56075, t995, t996);
        let t56115 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3044::<F>(t1100, t1102, t12190, t15562, t16612, t198, t3329, t3336, t336, t5023, t5024, t52762, t52806, t52808, t53011, t53056, t53107, t53163, t53217, t54238, t54240, t54242, t54245, t55405, t55453, t56099);
        let t56137 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3045::<F>(t30, t265, t393, t51814, t52167, t52197, t52227, t52870, t52883, t52906, t52924, t56115, t10326, t1106, t11095, t12201, t13312, t1468, t1469, t15083, t1587, t16618, t1704, t2257, t2258, t3340, t395, t4186, t45, t4560, t49889, t5028, t51827, t51829, t51831, t51833, t51835, t605, t606, t9344, dens_threshold, rho0, zeta_threshold);
        let (t56149, t56151) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3046::<F>(t10356, t16714, t128, t3360);
        let (t56153, t56155) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3047::<F>(t16737, t2258, t1120, t128);
        let (t56157, t56159) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3048::<F>(t16724, t2251, t1120, t128);
        let (t56161, t56163) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3049::<F>(t10326, t5051, t1120, t128);
    (t56137, t56149, t56151, t56153, t56155, t56157, t56159, t56161, t56163)
}
