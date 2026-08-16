//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta493 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1512;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1513;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1514;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1515;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1516;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1517;
use chunk6::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1518;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta493<F: Float>(t225: F, t80048: F, t6387: F, t3792: F, t40046: F, t12250: F, t550: F, t12419: F, t16224: F, t16305: F, t16394: F, t1825: F, t19871: F, t19876: F, t19956: F, t20442: F, t20460: F, t20470: F, t20473: F, t20563: F, t28099: F, t3803: F, t3805: F, t5246: F, t5248: F, t6330: F, t6347: F, t6388: F, t6394: F, t6420: F, t74090: F, t74120: F, t16233: F, t1799: F, t20416: F, t20448: F, t20450: F, t20454: F, t20463: F, t20465: F, t5249: F, t56878: F, t6396: F, t74110: F, t74147: F, t74189: F, t74415: F, t75008: F, t1363: F, t1367: F, t19904: F, t20433: F, t3870: F, t40070: F, t5240: F, t53901: F, t6427: F, t6431: F, t74191: F, t74212: F, t74214: F, t74217: F, t74228: F, t74256: F, t79921: F, t79984: F, t80021: F, t820: F, t20468: F, t39936: F, t74258: F, t74260: F, t74274: F, t74276: F, t74297: F, t74299: F, t74360: F, t74376: F, t74393: F, t119: F, t1315: F, t1831: F, t20479: F, t210: F, t554: F, t559: F, t56795: F, t74311: F, t74395: F, t74401: F, t74403: F, t74405: F, t74578: F, t74584: F, t74597: F, t74618: F, t16311: F, t20475: F, t3733: F, t40025: F, t54151: F, t56927: F, t56946: F, t56953: F, t56993: F, t57011: F, t57019: F, t57041: F, t57073: F, t12215: F, t12351: F, t1341: F, t1343: F, t20500: F, t20565: F, t3790: F, t40044: F, t40168: F, t54582: F, t57033: F, t57310: F, t57383: F, t6370: F, t6390: F, t74592: F, t80151: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t80175, t80181, t80185, t80189, t80193, t80265) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1512::<F>(t225, t80048, t6387, t3792, t40046, t12250, t550, t12419, t16224, t16305, t16394, t1825, t19871, t19876, t19956, t20442, t20460, t20470, t20473, t20563, t28099, t3803, t3805, t5246, t5248, t6330, t6347, t6388, t6394, t6420, t74090, t74120);
        let t80303 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1513::<F>(t12250, t12419, t16233, t16305, t16394, t1799, t19871, t19956, t20416, t20448, t20450, t20454, t20463, t20465, t3803, t3805, t5248, t5249, t550, t56878, t6394, t6396, t74110, t74120, t74147, t74189, t74415, t75008);
        let t80330 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1514::<F>(t1363, t1367, t19904, t20433, t3870, t40070, t5240, t53901, t6427, t6431, t74191, t74212, t74214, t74217, t74228, t74256, t79921, t79984, t80021, t820);
        let t80352 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1515::<F>(t12419, t19871, t19956, t20448, t20463, t20468, t3803, t3805, t39936, t5246, t74120, t74258, t74260, t74274, t74276, t74297, t74299, t74360, t74376, t74393);
        let t80375 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1516::<F>(t119, t1315, t1831, t20479, t210, t5240, t554, t559, t56795, t74311, t74395, t74401, t74403, t74405, t74578, t74584, t74597, t74618, t79984, t80175);
        let t80399 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1517::<F>(t119, t16311, t19876, t20475, t210, t3733, t40025, t5246, t5248, t54151, t56927, t56946, t56953, t56993, t57011, t57019, t57041, t57073, t74090, t79921, t80021);
        let t80442 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1518::<F>(t12215, t12351, t1341, t1343, t1363, t1799, t1825, t20416, t20500, t20565, t210, t3733, t3790, t3803, t3870, t40044, t40168, t5240, t54582, t57033, t57310, t57383, t6330, t6347, t6370, t6390, t74592, t80151, t80181, t80185, t820);
    (t80175, t80181, t80185, t80189, t80193, t80265, t80303, t80330, t80352, t80375, t80399, t80442)
}
