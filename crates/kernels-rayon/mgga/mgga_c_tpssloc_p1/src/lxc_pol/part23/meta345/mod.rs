//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta345 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1133;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1134;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1135;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1136;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1137;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta345(t204: f64, t2368: f64, t2459: f64, t2462: f64, t2471: f64, t2472: f64, t2476: f64, t2480: f64, t2490: f64, t2494: f64, t2495: f64, t2505: f64, t2509: f64, t2513: f64, t268: f64, t39373: f64, t39389: f64, t39397: f64, t39400: f64, t39408: f64, t39411: f64, t676: f64, t746: f64, t9489: f64, t9729: f64, t9734: f64, t9739: f64, t9755: f64, t9759: f64, t9766: f64, t9803: f64, t9810: f64, t9814: f64, t118: f64, t159: f64, t168: f64, t2458: f64, t2461: f64, t2475: f64, t2479: f64, t2504: f64, t2510: f64, t2512: f64, t39273: f64, t39275: f64, t39278: f64, t39281: f64, t39283: f64, t39284: f64, t39289: f64, t39291: f64, t39293: f64, t39295: f64, t39298: f64, t39378: f64, t39463: f64, t39468: f64, t39472: f64, t39476: f64, t39483: f64, t39664: f64, t690: f64, t725: f64, t730: f64, t731: f64, t9730: f64, t9733: f64, t9758: f64, t9892: f64, t9905: f64, t181: f64, t2369: f64, t2460: f64, t2477: f64, t39263: f64, t39529: f64, t39549: f64, t39563: f64, t39585: f64, t39590: f64, t39593: f64, t39658: f64, t745: f64, t747: f64, t9711: f64, t9751: f64, t9752: f64, t9762: f64, t9843: f64, t39706: f64, t17: f64, t521: f64, t1287: f64, t9216: f64, t11985: f64, t25: f64, t514: f64, t11998: f64, t28: f64, t517: f64, t32253: f64, t59: f64, t154: f64, t541: f64, t12289: f64, t1336: f64, t835: f64, t1314: f64, t9569: f64, t2559: f64, t3732: f64, t12214: f64, t782: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t39749 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1133(t204, t2368, t2459, t2462, t2471, t2472, t2476, t2480, t2490, t2494, t2495, t2505, t2509, t2513, t268, t39373, t39389, t39397, t39400, t39408, t39411, t676, t746, t9489, t9729, t9734, t9739, t9755, t9759, t9766, t9803, t9810, t9814);
        let t39803 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1134(t118, t159, t168, t2458, t2459, t2461, t2471, t2472, t2475, t2476, t2479, t2495, t2504, t2510, t2512, t39273, t39275, t39278, t39281, t39283, t39284, t39289, t39291, t39293, t39295, t39298, t39378, t39389, t39463, t39468, t39472, t39476, t39483, t39664, t690, t725, t730, t731, t9730, t9733, t9739, t9758, t9892, t9905);
        let t39840 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1135(t2471, t118, t181, t2369, t2460, t2462, t2477, t2479, t2494, t2510, t2512, t39263, t39283, t39529, t39549, t39563, t39585, t39590, t39593, t39658, t39664, t730, t731, t745, t747, t9711, t9730, t9751, t9752, t9758, t9762, t9843);
        let (t39842, t39844, t39856, t39861, t39877) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1136(t39706, t39749, t39803, t39840, t17, t521, t1287, t9216, t11985, t25, t514, t11998, t28, t517);
        let (t39933, t39934, t39936, t39944, t40005, t40018, t40021) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1137(t32253, t59, t154, t541, t12289, t1336, t835, t1314, t9569, t2559, t3732, t12214, t782);
    (t39842, t39844, t39856, t39861, t39877, t39933, t39934, t39936, t39944, t40005, t40018, t40021)
}
