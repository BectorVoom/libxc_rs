//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta345 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1133;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1134;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1135;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1136;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1137;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta345<F: Float>(t204: F, t2368: F, t2459: F, t2462: F, t2471: F, t2472: F, t2476: F, t2480: F, t2490: F, t2494: F, t2495: F, t2505: F, t2509: F, t2513: F, t268: F, t39373: F, t39389: F, t39397: F, t39400: F, t39408: F, t39411: F, t676: F, t746: F, t9489: F, t9729: F, t9734: F, t9739: F, t9755: F, t9759: F, t9766: F, t9803: F, t9810: F, t9814: F, t118: F, t159: F, t168: F, t2458: F, t2461: F, t2475: F, t2479: F, t2504: F, t2510: F, t2512: F, t39273: F, t39275: F, t39278: F, t39281: F, t39283: F, t39284: F, t39289: F, t39291: F, t39293: F, t39295: F, t39298: F, t39378: F, t39463: F, t39468: F, t39472: F, t39476: F, t39483: F, t39664: F, t690: F, t725: F, t730: F, t731: F, t9730: F, t9733: F, t9758: F, t9892: F, t9905: F, t181: F, t2369: F, t2460: F, t2477: F, t39263: F, t39529: F, t39549: F, t39563: F, t39585: F, t39590: F, t39593: F, t39658: F, t745: F, t747: F, t9711: F, t9751: F, t9752: F, t9762: F, t9843: F, t39706: F, t17: F, t521: F, t1287: F, t9216: F, t11985: F, t25: F, t514: F, t11998: F, t28: F, t517: F, t32253: F, t59: F, t154: F, t541: F, t12289: F, t1336: F, t835: F, t1314: F, t9569: F, t2559: F, t3732: F, t12214: F, t782: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t39749 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1133::<F>(t204, t2368, t2459, t2462, t2471, t2472, t2476, t2480, t2490, t2494, t2495, t2505, t2509, t2513, t268, t39373, t39389, t39397, t39400, t39408, t39411, t676, t746, t9489, t9729, t9734, t9739, t9755, t9759, t9766, t9803, t9810, t9814);
        let t39803 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1134::<F>(t118, t159, t168, t2458, t2459, t2461, t2471, t2472, t2475, t2476, t2479, t2495, t2504, t2510, t2512, t39273, t39275, t39278, t39281, t39283, t39284, t39289, t39291, t39293, t39295, t39298, t39378, t39389, t39463, t39468, t39472, t39476, t39483, t39664, t690, t725, t730, t731, t9730, t9733, t9739, t9758, t9892, t9905);
        let t39840 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1135::<F>(t2471, t118, t181, t2369, t2460, t2462, t2477, t2479, t2494, t2510, t2512, t39263, t39283, t39529, t39549, t39563, t39585, t39590, t39593, t39658, t39664, t730, t731, t745, t747, t9711, t9730, t9751, t9752, t9758, t9762, t9843);
        let (t39842, t39844, t39856, t39861, t39877) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1136::<F>(t39706, t39749, t39803, t39840, t17, t521, t1287, t9216, t11985, t25, t514, t11998, t28, t517);
        let (t39933, t39934, t39936, t39944, t40005, t40018, t40021) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1137::<F>(t32253, t59, t154, t541, t12289, t1336, t835, t1314, t9569, t2559, t3732, t12214, t782);
    (t39842, t39844, t39856, t39861, t39877, t39933, t39934, t39936, t39944, t40005, t40018, t40021)
}
