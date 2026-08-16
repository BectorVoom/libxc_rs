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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta821(t1086: f64, t4930: f64, t994: f64, t342: f64, t378: f64, t43471: f64, t3154: f64, t43350: f64, t16565: f64, t989: f64, t1071: f64, t12046: f64, t3298: f64, t4743: f64, t1024: f64, t11247: f64, t11788: f64, t11902: f64, t12105: f64, t15648: f64, t15655: f64, t16414: f64, t16485: f64, t16569: f64, t1692: f64, t3278: f64, t3288: f64, t3291: f64, t3295: f64, t3305: f64, t3317: f64, t3318: f64, t53670: f64, t53877: f64, t55880: f64, t3316: f64, t19602: f64, t19607: f64, t1082: f64, t1087: f64, t1089: f64, t1093: f64, t11620: f64, t12047: f64, t12052: f64, t12070: f64, t12086: f64, t12100: f64, t12124: f64, t12128: f64, t15886: f64, t16496: f64, t16509: f64, t16566: f64, t16568: f64, t1678: f64, t3204: f64, t3319: f64, t381: f64, t4857: f64, t4954: f64, t52977: f64, t54112: f64, t54479: f64, t55586: f64, t12166: f64, t1647: f64, t1043: f64, t11782: f64, t12032: f64, t12074: f64, t12137: f64, t12169: f64, t15717: f64, t16450: f64, t16552: f64, t16554: f64, t16559: f64, t16561: f64, t1668: f64, t1685: f64, t3223: f64, t3302: f64, t357: f64, t42278: f64, t43446: f64, t4893: f64, t4981: f64, t4983: f64, t4996: f64, t5005: f64, t53904: f64, t54909: f64, t55499: f64, t4746: f64, t4980: f64, t11202: f64, t11940: f64, t12111: f64, t12157: f64, t16152: f64, t16381: f64, t16402: f64, t16529: f64, t16544: f64, t3287: f64, t3313: f64, t43378: f64, t4964: f64, t4967: f64, t5004: f64, t53192: f64, t53683: f64, t54249: f64, t379: f64, t1000: f64, t1076: f64, t1079: f64, t1096: f64, t1097: f64, t11128: f64, t11174: f64, t11177: f64, t11187: f64, t11210: f64, t11220: f64, t12173: f64, t15579: f64, t16295: f64, t16314: f64, t16340: f64, t1651: f64, t16591: f64, t16592: f64, t1696: f64, t3047: f64, t3052: f64, t3058: f64, t3060: f64, t3269: f64, t3271: f64, t41993: f64, t42041: f64, t4773: f64, t4778: f64, t4947: f64, t5016: f64, t53108: f64, t55458: f64, t55461: f64, t55464: f64, t55475: f64, t55524: f64, t55562: f64, t55607: f64, t55643: f64, t55676: f64, t55711: f64, t55746: f64, t55783: f64, t55822: f64, t55854: f64, t55894: f64, t55926: f64, t995: f64, t996: f64, t1100: f64, t1102: f64, t12190: f64, t15562: f64, t16612: f64, t198: f64, t3329: f64, t3336: f64, t336: f64, t5023: f64, t5024: f64, t52762: f64, t52806: f64, t52808: f64, t53011: f64, t53056: f64, t53107: f64, t53163: f64, t53217: f64, t54238: f64, t54240: f64, t54242: f64, t54245: f64, t55405: f64, t55453: f64, t30: f64, t265: f64, t393: f64, t51814: f64, t52167: f64, t52197: f64, t52227: f64, t52870: f64, t52883: f64, t52906: f64, t52924: f64, t10326: f64, t1106: f64, t11095: f64, t12201: f64, t13312: f64, t1468: f64, t1469: f64, t15083: f64, t1587: f64, t16618: f64, t1704: f64, t2257: f64, t2258: f64, t3340: f64, t395: f64, t4186: f64, t45: f64, t4560: f64, t49889: f64, t5028: f64, t51827: f64, t51829: f64, t51831: f64, t51833: f64, t51835: f64, t605: f64, t606: f64, t9344: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t10356: f64, t16714: f64, t128: f64, t3360: f64, t16737: f64, t1120: f64, t16724: f64, t2251: f64, t5051: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t55934, t55938, t55939, t55944, t55948) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3038(t1086, t4930, t994, t342, t378, t43471, t3154, t43350, t16565, t989, t1071, t12046);
        let t55966 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3039(t3298, t4743, t1024, t11247, t11788, t11902, t12105, t15648, t15655, t16414, t16485, t16569, t1692, t3278, t3288, t3291, t3295, t3305, t3317, t3318, t53670, t53877, t55880, t55934, t55938, t55939, t55944, t55948);
        let t56001 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3040(t3316, t4743, t19602, t994, t19607, t1082, t1087, t1089, t1093, t11620, t12047, t12052, t12070, t12086, t12100, t12124, t12128, t15886, t16496, t16509, t16566, t16568, t1678, t3204, t3278, t3319, t381, t4857, t4954, t52977, t54112, t54479, t55586);
        let t56041 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3041(t12166, t1647, t1043, t1087, t1089, t11620, t11782, t12032, t12074, t12137, t12169, t15717, t16450, t16552, t16554, t16559, t16561, t1668, t1685, t3223, t3302, t357, t42278, t43446, t4857, t4893, t4954, t4981, t4983, t4996, t5005, t53904, t54479, t54909, t55499);
        let t56075 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3042(t4746, t4980, t1082, t1087, t1089, t11202, t11782, t11788, t11940, t12111, t12124, t12157, t16152, t16381, t16402, t16529, t16544, t3204, t3287, t3291, t3313, t378, t43378, t4857, t4964, t4967, t5004, t53192, t53683, t54249, t989);
        let t56099 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3043(t342, t379, t1000, t1076, t1079, t1096, t1097, t11128, t11174, t11177, t11187, t11210, t11220, t12173, t15579, t15648, t16295, t16314, t16340, t1651, t16591, t16592, t1696, t3047, t3052, t3058, t3060, t3269, t3271, t41993, t42041, t4773, t4778, t4947, t5016, t53108, t54112, t55458, t55461, t55464, t55475, t55524, t55562, t55607, t55643, t55676, t55711, t55746, t55783, t55822, t55854, t55894, t55926, t55966, t56001, t56041, t56075, t995, t996);
        let t56115 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3044(t1100, t1102, t12190, t15562, t16612, t198, t3329, t3336, t336, t5023, t5024, t52762, t52806, t52808, t53011, t53056, t53107, t53163, t53217, t54238, t54240, t54242, t54245, t55405, t55453, t56099);
        let t56137 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3045(t30, t265, t393, t51814, t52167, t52197, t52227, t52870, t52883, t52906, t52924, t56115, t10326, t1106, t11095, t12201, t13312, t1468, t1469, t15083, t1587, t16618, t1704, t2257, t2258, t3340, t395, t4186, t45, t4560, t49889, t5028, t51827, t51829, t51831, t51833, t51835, t605, t606, t9344, dens_threshold, rho0, zeta_threshold);
        let (t56149, t56151) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3046(t10356, t16714, t128, t3360);
        let (t56153, t56155) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3047(t16737, t2258, t1120, t128);
        let (t56157, t56159) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3048(t16724, t2251, t1120, t128);
        let (t56161, t56163) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3049(t10326, t5051, t1120, t128);
    (t56137, t56149, t56151, t56153, t56155, t56157, t56159, t56161, t56163)
}
