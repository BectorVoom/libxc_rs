//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta482 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1451;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1452;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1453;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1454;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1455;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1456;
use chunk6::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1457;
use chunk7::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1458;
use chunk8::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1459;
use chunk9::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1460;
use chunk10::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1461;
use chunk11::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1462;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta482(t6218: f64, t11668: f64, t11779: f64, t1214: f64, t1227: f64, t15615: f64, t1735: f64, t1748: f64, t19033: f64, t21745: f64, t21749: f64, t22197: f64, t22208: f64, t248: f64, t3506: f64, t3508: f64, t3577: f64, t3578: f64, t4582: f64, t47: f64, t471: f64, t479: f64, t488: f64, t5005: f64, t6207: f64, t65600: f64, t65605: f64, t72255: f64, t72352: f64, t72366: f64, t77606: f64, t77957: f64, t8025: f64, t78118: f64, t78120: f64, t78122: f64, t78125: f64, t78128: f64, t78132: f64, t78196: f64, t78199: f64, t78227: f64, t78229: f64, t78232: f64, t78236: f64, t78239: f64, t78242: f64, t78247: f64, t78250: f64, t78254: f64, t78281: f64, t78283: f64, t78286: f64, t78291: f64, t78294: f64, t78296: f64, t78298: f64, t78302: f64, t78304: f64, t78310: f64, t78312: f64, t78314: f64, t78318: f64, t78320: f64, t78327: f64, t78329: f64, t78331: f64, t78333: f64, t50834: f64, t71335: f64, t71337: f64, t77959: f64, t77963: f64, t77967: f64, t77971: f64, t77975: f64, t77979: f64, t77983: f64, t77989: f64, t77992: f64, t77995: f64, t77998: f64, t63332: f64, t63334: f64, t63888: f64, t63893: f64, t63911: f64, t71142: f64, t71144: f64, t71146: f64, t71152: f64, t71154: f64, t71156: f64, t71408: f64, t78002: f64, t78005: f64, t44249: f64, t50846: f64, t71470: f64, t71472: f64, t71474: f64, t78026: f64, t78029: f64, t78033: f64, t78037: f64, t78041: f64, t78045: f64, t78049: f64, t78078: f64, t78080: f64, t44275: f64, t63361: f64, t78057: f64, t78084: f64, t78087: f64, t78090: f64, t78093: f64, t78095: f64, t78097: f64, t78100: f64, t78103: f64, t78105: f64, t78107: f64, t78109: f64, t6036: f64, t1129: f64, t11365: f64, t1137: f64, t1156: f64, t15126: f64, t21947: f64, t3376: f64, t3401: f64, t3403: f64, t44177: f64, t44179: f64, t78243: f64, t78287: f64, t11285: f64, t11350: f64, t11352: f64, t1148: f64, t1683: f64, t1695: f64, t18840: f64, t18899: f64, t21855: f64, t21887: f64, t21890: f64, t21939: f64, t21942: f64, t3359: f64, t43692: f64, t44155: f64, t44223: f64, t44361: f64, t4797: f64, t4835: f64, t51376: f64, t51427: f64, t51604: f64, t6037: f64, t6053: f64, t6056: f64, t6085: f64, t6088: f64, t63602: f64, t64103: f64, t64292: f64, t71860: f64, t71863: f64, t78114: f64, t11310: f64, t11420: f64, t15146: f64, t15207: f64, t1682: f64, t1694: f64, t18622: f64, t21839: f64, t21842: f64, t21845: f64, t3332: f64, t6052: f64, t6069: f64, t6084: f64, t71672: f64, t78225: f64, t78335: f64, t78355: f64, t44320: f64, t15136: f64, t18650: f64, t21836: f64, t21907: f64, t21952: f64, t3357: f64, t436: f64, t51680: f64, t63454: f64, t71729: f64, t78359: f64, t78361: f64, t78364: f64, t78367: f64, t78370: f64, t78373: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t78757, t78775) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1451(t6218, t11668, t11779, t1214, t1227, t15615, t1735, t1748, t19033, t21745, t21749, t22197, t22208, t248, t3506, t3508, t3577, t3578, t4582, t47, t471, t479, t488, t5005, t6207, t65600, t65605, t72255, t72352, t72366, t77606, t77957, t8025);
        let (t78791, t78792) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1452(t78118, t78120, t78122, t78125, t78128, t78132, t78196, t78199, t78227, t78229, t78232, t78236, t78239, t78242, t78247, t78250, t78254, t78281, t78283, t78286, t78291, t78294, t78296);
        let t78794 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1453(t78298, t78302, t78304, t78310, t78312, t78314, t78318, t78320, t78327, t78329, t78331, t78333);
        let t78809 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1454(t50834, t71335, t71337, t77959, t77963, t77967, t77971, t77975, t77979, t77983, t77989, t77992, t77995, t77998);
        let t78824 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1455(t63332, t63334, t63888, t63893, t63911, t71142, t71144, t71146, t71152, t71154, t71156, t71408, t78002, t78005);
        let t78839 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1456(t44249, t50846, t71470, t71472, t71474, t78026, t78029, t78033, t78037, t78041, t78045, t78049, t78078, t78080);
        let t78853 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1457(t44275, t63361, t78057, t78084, t78087, t78090, t78093, t78095, t78097, t78100, t78103, t78105, t78107, t78109);
        let (t78859, t78874) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1458(t6036, t1129, t11365, t1137, t1156, t15126, t21947, t3376, t3401, t3403, t44177, t44179, t78132, t78196, t78199, t78229, t78232, t78236, t78239, t78243, t78281, t78283, t78286, t78287, t78298, t78809, t78824, t78839, t78853);
        let t78914 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1459(t11285, t11350, t11352, t1148, t1156, t1683, t1695, t18840, t18899, t21855, t21887, t21890, t21939, t21942, t3359, t43692, t44155, t44223, t44361, t4797, t4835, t51376, t51427, t51604, t6037, t6053, t6056, t6085, t6088, t63602, t64103, t64292, t71860, t71863, t78114, t78287, t78859);
        let t78944 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1460(t11310, t11365, t11420, t15126, t15146, t15207, t1682, t1694, t18622, t21839, t21842, t21845, t21887, t21939, t3332, t3376, t3401, t6052, t6056, t6069, t6084, t6088, t71672, t78225, t78327, t78329, t78331, t78333, t78335, t78355);
        let (t78961, t78973) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1461(t63332, t63334, t63361, t71142, t71144, t71146, t71152, t77989, t77992, t77995, t78057, t44320, t50834, t71154, t71156, t77998, t78002, t78005, t78033, t78037, t78041, t78045, t78049);
        let t79002 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1462(t6052, t11310, t11350, t1137, t11420, t15136, t15146, t1682, t18650, t21836, t21907, t21952, t3332, t3357, t3359, t3403, t436, t51680, t6037, t6069, t63454, t71729, t78287, t78359, t78361, t78364, t78367, t78370, t78373, t78859, t78961, t78973);
    (t78757, t78775, t78791, t78792, t78794, t78874, t78914, t78944, t79002)
}
