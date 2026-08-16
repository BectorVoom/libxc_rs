//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta493 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1512;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1513;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1514;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1515;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1516;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1517;
use chunk6::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1518;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta493(t225: f64, t80048: f64, t6387: f64, t3792: f64, t40046: f64, t12250: f64, t550: f64, t12419: f64, t16224: f64, t16305: f64, t16394: f64, t1825: f64, t19871: f64, t19876: f64, t19956: f64, t20442: f64, t20460: f64, t20470: f64, t20473: f64, t20563: f64, t28099: f64, t3803: f64, t3805: f64, t5246: f64, t5248: f64, t6330: f64, t6347: f64, t6388: f64, t6394: f64, t6420: f64, t74090: f64, t74120: f64, t16233: f64, t1799: f64, t20416: f64, t20448: f64, t20450: f64, t20454: f64, t20463: f64, t20465: f64, t5249: f64, t56878: f64, t6396: f64, t74110: f64, t74147: f64, t74189: f64, t74415: f64, t75008: f64, t1363: f64, t1367: f64, t19904: f64, t20433: f64, t3870: f64, t40070: f64, t5240: f64, t53901: f64, t6427: f64, t6431: f64, t74191: f64, t74212: f64, t74214: f64, t74217: f64, t74228: f64, t74256: f64, t79921: f64, t79984: f64, t80021: f64, t820: f64, t20468: f64, t39936: f64, t74258: f64, t74260: f64, t74274: f64, t74276: f64, t74297: f64, t74299: f64, t74360: f64, t74376: f64, t74393: f64, t119: f64, t1315: f64, t1831: f64, t20479: f64, t210: f64, t554: f64, t559: f64, t56795: f64, t74311: f64, t74395: f64, t74401: f64, t74403: f64, t74405: f64, t74578: f64, t74584: f64, t74597: f64, t74618: f64, t16311: f64, t20475: f64, t3733: f64, t40025: f64, t54151: f64, t56927: f64, t56946: f64, t56953: f64, t56993: f64, t57011: f64, t57019: f64, t57041: f64, t57073: f64, t12215: f64, t12351: f64, t1341: f64, t1343: f64, t20500: f64, t20565: f64, t3790: f64, t40044: f64, t40168: f64, t54582: f64, t57033: f64, t57310: f64, t57383: f64, t6370: f64, t6390: f64, t74592: f64, t80151: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t80175, t80181, t80185, t80189, t80193, t80265) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1512(t225, t80048, t6387, t3792, t40046, t12250, t550, t12419, t16224, t16305, t16394, t1825, t19871, t19876, t19956, t20442, t20460, t20470, t20473, t20563, t28099, t3803, t3805, t5246, t5248, t6330, t6347, t6388, t6394, t6420, t74090, t74120);
        let t80303 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1513(t12250, t12419, t16233, t16305, t16394, t1799, t19871, t19956, t20416, t20448, t20450, t20454, t20463, t20465, t3803, t3805, t5248, t5249, t550, t56878, t6394, t6396, t74110, t74120, t74147, t74189, t74415, t75008);
        let t80330 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1514(t1363, t1367, t19904, t20433, t3870, t40070, t5240, t53901, t6427, t6431, t74191, t74212, t74214, t74217, t74228, t74256, t79921, t79984, t80021, t820);
        let t80352 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1515(t12419, t19871, t19956, t20448, t20463, t20468, t3803, t3805, t39936, t5246, t74120, t74258, t74260, t74274, t74276, t74297, t74299, t74360, t74376, t74393);
        let t80375 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1516(t119, t1315, t1831, t20479, t210, t5240, t554, t559, t56795, t74311, t74395, t74401, t74403, t74405, t74578, t74584, t74597, t74618, t79984, t80175);
        let t80399 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1517(t119, t16311, t19876, t20475, t210, t3733, t40025, t5246, t5248, t54151, t56927, t56946, t56953, t56993, t57011, t57019, t57041, t57073, t74090, t79921, t80021);
        let t80442 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1518(t12215, t12351, t1341, t1343, t1363, t1799, t1825, t20416, t20500, t20565, t210, t3733, t3790, t3803, t3870, t40044, t40168, t5240, t54582, t57033, t57310, t57383, t6330, t6347, t6370, t6390, t74592, t80151, t80181, t80185, t820);
    (t80175, t80181, t80185, t80189, t80193, t80265, t80303, t80330, t80352, t80375, t80399, t80442)
}
