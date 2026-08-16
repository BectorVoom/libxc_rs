//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta521 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2050;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2051;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2052;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2053;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2054;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2055;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta521(t2223: f64, t3824: f64, t2475: f64, t2461: f64, t2478: f64, t159: f64, t172: f64, t2454: f64, t268: f64, t39249: f64, t39256: f64, t39300: f64, t39309: f64, t39312: f64, t39316: f64, t39320: f64, t39377: f64, t39378: f64, t39381: f64, t39535: f64, t676: f64, t724: f64, t732: f64, t739: f64, t740: f64, t746: f64, t747: f64, t781: f64, t9493: f64, t9720: f64, t9738: f64, t9740: f64, t9752: f64, t9762: f64, t9763: f64, t9781: f64, t9828: f64, t204: f64, t2368: f64, t2459: f64, t2462: f64, t2471: f64, t2472: f64, t2476: f64, t2480: f64, t2490: f64, t2494: f64, t2495: f64, t2505: f64, t2509: f64, t2513: f64, t39373: f64, t39389: f64, t39397: f64, t39400: f64, t39408: f64, t39411: f64, t9489: f64, t9729: f64, t9734: f64, t9739: f64, t9755: f64, t9759: f64, t9766: f64, t9803: f64, t9810: f64, t9814: f64, t118: f64, t168: f64, t2458: f64, t2479: f64, t2504: f64, t2510: f64, t2512: f64, t39273: f64, t39275: f64, t39278: f64, t39281: f64, t39283: f64, t39284: f64, t39289: f64, t39291: f64, t39293: f64, t39295: f64, t39298: f64, t39463: f64, t39468: f64, t39472: f64, t39476: f64, t39483: f64, t690: f64, t725: f64, t730: f64, t731: f64, t9730: f64, t9733: f64, t9758: f64, t9892: f64, t9905: f64, t181: f64, t2369: f64, t2460: f64, t2477: f64, t39263: f64, t39529: f64, t39549: f64, t39563: f64, t39585: f64, t39590: f64, t39593: f64, t39658: f64, t745: f64, t9711: f64, t9751: f64, t9843: f64, t17: f64, t521: f64, t2225: f64, t3826: f64, t12129: f64, t592: f64, t1287: f64, t9216: f64, t11985: f64, t25: f64, t514: f64, t11998: f64, t28: f64, t517: f64, t12442: f64, t225: f64, t12036: f64, t12016: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t39659, t39664, t39706) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2050(t2223, t3824, t2475, t2461, t2478, t159, t172, t2454, t268, t39249, t39256, t39300, t39309, t39312, t39316, t39320, t39377, t39378, t39381, t39535, t676, t724, t732, t739, t740, t746, t747, t781, t9493, t9720, t9738, t9740, t9752, t9762, t9763, t9781, t9828);
        let t39749 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2051(t204, t2368, t2459, t2462, t2471, t2472, t2476, t2480, t2490, t2494, t2495, t2505, t2509, t2513, t268, t39373, t39389, t39397, t39400, t39408, t39411, t676, t746, t9489, t9729, t9734, t9739, t9755, t9759, t9766, t9803, t9810, t9814);
        let t39803 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2052(t118, t159, t168, t2458, t2459, t2461, t2471, t2472, t2475, t2476, t2479, t2495, t2504, t2510, t2512, t39273, t39275, t39278, t39281, t39283, t39284, t39289, t39291, t39293, t39295, t39298, t39378, t39389, t39463, t39468, t39472, t39476, t39483, t39664, t690, t725, t730, t731, t9730, t9733, t9739, t9758, t9892, t9905);
        let t39840 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2053(t2471, t118, t181, t2369, t2460, t2462, t2477, t2479, t2494, t2510, t2512, t39263, t39283, t39529, t39549, t39563, t39585, t39590, t39593, t39658, t39664, t730, t731, t745, t747, t9711, t9730, t9751, t9752, t9758, t9762, t9843);
        let (t39842, t39844, t39845, t39851, t39855) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2054(t39706, t39749, t39803, t39840, t17, t521, t2225, t3826, t12129, t592, t1287, t9216);
        let (t39857, t39861, t39877, t39910, t39913, t39916) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2055(t2223, t3826, t11985, t25, t514, t11998, t28, t517, t12442, t225, t12036, t12016);
    (t39659, t39842, t39844, t39845, t39851, t39855, t39857, t39861, t39877, t39910, t39913, t39916)
}
