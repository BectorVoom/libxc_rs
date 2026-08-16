//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta323 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1143;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1144;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1145;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1146;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1147;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1148;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1149;
use chunk7::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1150;
use chunk8::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1151;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta323<F: Float>(t2223: F, t3824: F, t2475: F, t2461: F, t2478: F, t159: F, t172: F, t2454: F, t268: F, t39249: F, t39256: F, t39300: F, t39309: F, t39312: F, t39316: F, t39320: F, t39377: F, t39378: F, t39381: F, t39535: F, t676: F, t724: F, t732: F, t739: F, t740: F, t746: F, t747: F, t781: F, t9493: F, t9720: F, t9738: F, t9740: F, t9752: F, t9762: F, t9763: F, t9781: F, t9828: F, t204: F, t2368: F, t2459: F, t2462: F, t2471: F, t2472: F, t2476: F, t2480: F, t2490: F, t2494: F, t2495: F, t2505: F, t2509: F, t2513: F, t39373: F, t39389: F, t39397: F, t39400: F, t39408: F, t39411: F, t9489: F, t9729: F, t9734: F, t9739: F, t9755: F, t9759: F, t9766: F, t9803: F, t9810: F, t9814: F, t118: F, t168: F, t2458: F, t2479: F, t2504: F, t2510: F, t2512: F, t39273: F, t39275: F, t39278: F, t39281: F, t39283: F, t39284: F, t39289: F, t39291: F, t39293: F, t39295: F, t39298: F, t39463: F, t39468: F, t39472: F, t39476: F, t39483: F, t690: F, t725: F, t730: F, t731: F, t9730: F, t9733: F, t9758: F, t9892: F, t9905: F, t181: F, t2369: F, t2460: F, t2477: F, t39263: F, t39529: F, t39549: F, t39563: F, t39585: F, t39590: F, t39593: F, t39658: F, t745: F, t9711: F, t9751: F, t9843: F, t17: F, t521: F, t2225: F, t3826: F, t193: F, t23857: F, t3701: F, t3914: F, t39629: F, t39631: F, t39633: F, t39635: F, t39637: F, t39640: F, t39643: F, t39645: F, t39649: F, t39655: F, t5160: F, t533: F, t12129: F, t592: F, t184: F, t39454: F, t1287: F, t9216: F, t11985: F, t25: F, t514: F, t11987: F, t11991: F, t1298: F, t2249: F, t3665: F, t3704: F, t39109: F, t39420: F, t39426: F, t9257: F, t11998: F, t28: F, t517: F, zeta_threshold: F, t11122: F, t12000: F, t12004: F, t1302: F, t3231: F, t3673: F, t3711: F, t39437: F, t39443: F, t39448: F, t12442: F, t225: F, t12036: F, t12016: F, t12440: F, t3911: F, t12021: F, t12027: F, t12030: F, t12033: F, t12437: F, t12438: F, t12444: F, t1375: F, t1385: F, t1386: F, t3758: F, t3887: F, t3888: F, t3889: F, t3912: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t39660, t39664, t39706) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1143::<F>(t2223, t3824, t2475, t2461, t2478, t159, t172, t2454, t268, t39249, t39256, t39300, t39309, t39312, t39316, t39320, t39377, t39378, t39381, t39535, t676, t724, t732, t739, t740, t746, t747, t781, t9493, t9720, t9738, t9740, t9752, t9762, t9763, t9781, t9828);
        let t39749 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1144::<F>(t204, t2368, t2459, t2462, t2471, t2472, t2476, t2480, t2490, t2494, t2495, t2505, t2509, t2513, t268, t39373, t39389, t39397, t39400, t39408, t39411, t676, t746, t9489, t9729, t9734, t9739, t9755, t9759, t9766, t9803, t9810, t9814);
        let t39803 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1145::<F>(t118, t159, t168, t2458, t2459, t2461, t2471, t2472, t2475, t2476, t2479, t2495, t2504, t2510, t2512, t39273, t39275, t39278, t39281, t39283, t39284, t39289, t39291, t39293, t39295, t39298, t39378, t39389, t39463, t39468, t39472, t39476, t39483, t39664, t690, t725, t730, t731, t9730, t9733, t9739, t9758, t9892, t9905);
        let t39840 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1146::<F>(t2471, t118, t181, t2369, t2460, t2462, t2477, t2479, t2494, t2510, t2512, t39263, t39283, t39529, t39549, t39563, t39585, t39590, t39593, t39658, t39664, t730, t731, t745, t747, t9711, t9730, t9751, t9752, t9758, t9762, t9843);
        let (t39842, t39844, t39846, t39847) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1147::<F>(t39706, t39749, t39803, t39840, t17, t521, t2225, t3826, t193, t23857, t3701, t3914, t39629, t39631, t39633, t39635, t39637, t39640, t39643, t39645, t39649, t39655, t39658, t39660, t5160, t533);
        let (t39852, t39854, t39856, t39858, t39861) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1148::<F>(t12129, t592, t17, t184, t39454, t1287, t9216, t2223, t3826, t11985, t25, t514);
        let (t39874, t39877) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1149::<F>(t25, t11987, t11991, t1298, t2249, t3665, t3704, t39109, t39420, t39426, t39861, t9257, t11998, t28, t517, zeta_threshold);
        let t39892 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1150::<F>(t28, t11122, t12000, t12004, t1302, t3231, t3673, t3711, t39437, t39443, t39448, t39877, t39874, zeta_threshold);
        let t39932 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1151::<F>(t12442, t225, t12036, t12016, t12440, t3911, t12021, t12027, t12030, t12033, t12437, t12438, t12444, t1375, t1385, t1386, t3758, t3887, t3888, t3889, t3912);
    (t39660, t39842, t39844, t39846, t39847, t39852, t39854, t39856, t39858, t39892, t39932)
}
