//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta782 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2672;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2673;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2674;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2675;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2676;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2677;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2678;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta782<F: Float>(t54462: F, t39851: F, t54467: F, t57227: F, t57229: F, t57235: F, t40224: F, t40230: F, t54459: F, t54461: F, t54465: F, t54466: F, t54470: F, t54472: F, t54473: F, t54475: F, t54478: F, t1307: F, t1345: F, t1347: F, t1365: F, t16186: F, t16191: F, t16195: F, t1819: F, t19631: F, t19715: F, t19728: F, t19994: F, t20356: F, t20416: F, t20544: F, t20547: F, t20550: F, t5187: F, t5278: F, t5279: F, t546: F, t6347: F, t6924: F, t74355: F, t1348: F, t1821: F, t19702: F, t19708: F, t19716: F, t19719: F, t19725: F, t20536: F, t225: F, t5272: F, t5280: F, t5283: F, t548: F, t550: F, t6404: F, t6408: F, t6411: F, t68: F, t74466: F, t74467: F, t74469: F, t74471: F, t74480: F, t74487: F, t74498: F, t12286: F, t12351: F, t1341: F, t1343: F, t1363: F, t1799: F, t19921: F, t19926: F, t20497: F, t20556: F, t20565: F, t3778: F, t3783: F, t3870: F, t5240: F, t56776: F, t56779: F, t56795: F, t56797: F, t6330: F, t820: F, t1358: F, t20596: F, t12283: F, t20442: F, t120: F, t20465: F, t1351: F, t40046: F, t12429: F, t1352: F, t16224: F, t16233: F, t16305: F, t16306: F, t16394: F, t1825: F, t19744: F, t19876: F, t19945: F, t19976: F, t20004: F, t20450: F, t20463: F, t3803: F, t40168: F, t5246: F, t5248: F, t5308: F, t54048: F, t54744: F, t6388: F, t74120: F, t16398: F, t20470: F, t12419: F, t16242: F, t20448: F, t20500: F, t210: F, t3733: F, t54132: F, t54151: F, t56837: F, t56853: F, t56883: F, t56885: F, t56888: F, t56906: F, t56909: F, t56919: F, t56921: F, t16225: F, t16311: F, t5250: F, t54013: F, t54199: F, t56927: F, t56933: F, t56935: F, t56937: F, t56946: F, t56953: F, t56959: F, t56961: F, t56963: F, t56993: F, t57172: F, t74415: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t74499, t74500, t74501, t74502, t74503, t74504, t74505) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2672::<F>(t54462, t39851, t54467, t57227, t57229, t57235, t40224, t40230, t54459, t54461, t54465, t54466, t54470, t54472, t54473, t54475, t54478);
        let t74562 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2673::<F>(t1307, t1345, t1347, t1365, t16186, t16191, t16195, t1819, t19631, t19715, t19728, t19994, t20356, t20416, t20544, t20547, t20550, t5187, t5278, t5279, t546, t6347, t6924, t74355);
        let t74564 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2674::<F>(t1348, t1821, t19702, t19708, t19716, t19719, t19725, t20536, t225, t5272, t5280, t5283, t548, t550, t6404, t6408, t6411, t68, t74466, t74467, t74469, t74471, t74480, t74487, t74498, t74505, t74562);
        let t74569 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2675::<F>(t12286, t12351, t1307, t1341, t1343, t1363, t1799, t19631, t19921, t19926, t20416, t20497, t20556, t20565, t3778, t3783, t3870, t5187, t5240, t56776, t56779, t56795, t56797, t6330, t6347, t74564, t820);
        let (t74599, t74610) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2676::<F>(t1358, t20596, t12283, t20442, t120, t20356, t20465, t1351, t40046, t12429, t1352, t16224, t16233, t16305, t16306, t16394, t1825, t19744, t19876, t19945, t19976, t19994, t20004, t20450, t20463, t3803, t40168, t5246, t5248, t5308, t54048, t54744, t6388, t74120);
        let t74632 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2677::<F>(t16398, t20470, t12419, t1307, t16242, t20448, t20500, t210, t3733, t3803, t54132, t54151, t56837, t56853, t56883, t56885, t56888, t56906, t56909, t56919, t56921);
        let t74655 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2678::<F>(t16224, t16225, t16305, t16311, t5246, t5250, t54013, t54199, t56927, t56933, t56935, t56937, t56946, t56953, t56959, t56961, t56963, t56993, t57172, t6388, t74415);
    (t74499, t74500, t74501, t74502, t74503, t74504, t74564, t74569, t74599, t74610, t74632, t74655)
}
