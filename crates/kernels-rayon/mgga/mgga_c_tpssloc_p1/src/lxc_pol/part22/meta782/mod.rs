//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta782 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2672;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2673;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2674;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2675;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2676;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2677;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2678;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta782(t54462: f64, t39851: f64, t54467: f64, t57227: f64, t57229: f64, t57235: f64, t40224: f64, t40230: f64, t54459: f64, t54461: f64, t54465: f64, t54466: f64, t54470: f64, t54472: f64, t54473: f64, t54475: f64, t54478: f64, t1307: f64, t1345: f64, t1347: f64, t1365: f64, t16186: f64, t16191: f64, t16195: f64, t1819: f64, t19631: f64, t19715: f64, t19728: f64, t19994: f64, t20356: f64, t20416: f64, t20544: f64, t20547: f64, t20550: f64, t5187: f64, t5278: f64, t5279: f64, t546: f64, t6347: f64, t6924: f64, t74355: f64, t1348: f64, t1821: f64, t19702: f64, t19708: f64, t19716: f64, t19719: f64, t19725: f64, t20536: f64, t225: f64, t5272: f64, t5280: f64, t5283: f64, t548: f64, t550: f64, t6404: f64, t6408: f64, t6411: f64, t68: f64, t74466: f64, t74467: f64, t74469: f64, t74471: f64, t74480: f64, t74487: f64, t74498: f64, t12286: f64, t12351: f64, t1341: f64, t1343: f64, t1363: f64, t1799: f64, t19921: f64, t19926: f64, t20497: f64, t20556: f64, t20565: f64, t3778: f64, t3783: f64, t3870: f64, t5240: f64, t56776: f64, t56779: f64, t56795: f64, t56797: f64, t6330: f64, t820: f64, t1358: f64, t20596: f64, t12283: f64, t20442: f64, t120: f64, t20465: f64, t1351: f64, t40046: f64, t12429: f64, t1352: f64, t16224: f64, t16233: f64, t16305: f64, t16306: f64, t16394: f64, t1825: f64, t19744: f64, t19876: f64, t19945: f64, t19976: f64, t20004: f64, t20450: f64, t20463: f64, t3803: f64, t40168: f64, t5246: f64, t5248: f64, t5308: f64, t54048: f64, t54744: f64, t6388: f64, t74120: f64, t16398: f64, t20470: f64, t12419: f64, t16242: f64, t20448: f64, t20500: f64, t210: f64, t3733: f64, t54132: f64, t54151: f64, t56837: f64, t56853: f64, t56883: f64, t56885: f64, t56888: f64, t56906: f64, t56909: f64, t56919: f64, t56921: f64, t16225: f64, t16311: f64, t5250: f64, t54013: f64, t54199: f64, t56927: f64, t56933: f64, t56935: f64, t56937: f64, t56946: f64, t56953: f64, t56959: f64, t56961: f64, t56963: f64, t56993: f64, t57172: f64, t74415: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t74499, t74500, t74501, t74502, t74503, t74504, t74505) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2672(t54462, t39851, t54467, t57227, t57229, t57235, t40224, t40230, t54459, t54461, t54465, t54466, t54470, t54472, t54473, t54475, t54478);
        let t74562 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2673(t1307, t1345, t1347, t1365, t16186, t16191, t16195, t1819, t19631, t19715, t19728, t19994, t20356, t20416, t20544, t20547, t20550, t5187, t5278, t5279, t546, t6347, t6924, t74355);
        let t74564 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2674(t1348, t1821, t19702, t19708, t19716, t19719, t19725, t20536, t225, t5272, t5280, t5283, t548, t550, t6404, t6408, t6411, t68, t74466, t74467, t74469, t74471, t74480, t74487, t74498, t74505, t74562);
        let t74569 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2675(t12286, t12351, t1307, t1341, t1343, t1363, t1799, t19631, t19921, t19926, t20416, t20497, t20556, t20565, t3778, t3783, t3870, t5187, t5240, t56776, t56779, t56795, t56797, t6330, t6347, t74564, t820);
        let (t74599, t74610) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2676(t1358, t20596, t12283, t20442, t120, t20356, t20465, t1351, t40046, t12429, t1352, t16224, t16233, t16305, t16306, t16394, t1825, t19744, t19876, t19945, t19976, t19994, t20004, t20450, t20463, t3803, t40168, t5246, t5248, t5308, t54048, t54744, t6388, t74120);
        let t74632 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2677(t16398, t20470, t12419, t1307, t16242, t20448, t20500, t210, t3733, t3803, t54132, t54151, t56837, t56853, t56883, t56885, t56888, t56906, t56909, t56919, t56921);
        let t74655 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2678(t16224, t16225, t16305, t16311, t5246, t5250, t54013, t54199, t56927, t56933, t56935, t56937, t56946, t56953, t56959, t56961, t56963, t56993, t57172, t6388, t74415);
    (t74499, t74500, t74501, t74502, t74503, t74504, t74564, t74569, t74599, t74610, t74632, t74655)
}
