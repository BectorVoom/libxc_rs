//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta767 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2645;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2646;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2647;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2648;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2649;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2650;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2651;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2652;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta767(t112: f64, t16506: f64, t111: f64, t5363: f64, t1851: f64, t3946: f64, t1858: f64, t3931: f64, t1395: f64, t5381: f64, t1404: f64, t6470: f64, t2319: f64, t4072: f64, t12521: f64, t12524: f64, t12813: f64, t1458: f64, t16535: f64, t19534: f64, t20181: f64, t2363: f64, t3941: f64, t45560: f64, t5371: f64, t5376: f64, t5456: f64, t5493: f64, t671: f64, t19530: f64, t626: f64, t12774: f64, t12795: f64, t12802: f64, t1447: f64, t16: f64, t19488: f64, t19489: f64, t19492: f64, t19499: f64, t19503: f64, t19504: f64, t19517: f64, t2219: f64, t2248: f64, t2336: f64, t2341: f64, t2351: f64, t2355: f64, t30171: f64, t30307: f64, t45697: f64, t45707: f64, t45751: f64, t45762: f64, t5469: f64, t5472: f64, t5475: f64, t657: f64, t659: f64, t92: f64, t2349: f64, t100: f64, t12792: f64, t12796: f64, t12799: f64, t12805: f64, t19493: f64, t19498: f64, t19513: f64, t19521: f64, t19525: f64, t21: f64, t2342: f64, t2350: f64, t2354: f64, t4049: f64, t4059: f64, t45460: f64, t45496: f64, t45717: f64, t5396: f64, t5468: f64, t5480: f64, t5484: f64, t584: f64, t662: f64, t9: f64, t9384: f64, t9398: f64, t4067: f64, t2331: f64, t45421: f64, t45422: f64, t45424: f64, t45426: f64, t45656: f64, t45658: f64, t45660: f64, t45662: f64, t45688: f64, t45690: f64, t64: f64, t656: f64, t2281: f64, t5489: f64, t5465: f64, t19474: f64, t19483: f64, t19477: f64, t12808: f64, t19473: f64, t19482: f64, t19529: f64, t2332: f64, t2358: f64, t26129: f64, t29903: f64, t4043: f64, t45435: f64, t45676: f64, t5464: f64, t5488: f64, t666: f64, t9365: f64, t109: f64, t576: f64, t12649: f64, t12652: f64, t12653: f64, t12656: f64, t12661: f64, t12708: f64, t1410: f64, t1426: f64, t1434: f64, t19343: f64, t19346: f64, t19349: f64, t19441: f64, t2304: f64, t3961: f64, t3962: f64, t3967: f64, t3997: f64, t4018: f64, t5403: f64, t609: f64, t642: f64, t80: f64, t1409: f64, t628: f64, t67: f64, t2250: f64, t5398: f64, t16558: f64, t607: f64, t12606: f64, t12620: f64, t12623: f64, t12662: f64, t12665: f64, t1411: f64, t17635: f64, t1864: f64, t19322: f64, t19323: f64, t19363: f64, t19404: f64, t2251: f64, t3966: f64, t3968: f64, t3971: f64, t5427: f64, t608: f64, t65: f64, t6509: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t55341, t55353, t55368, t55374, t55376, t55378, t55388) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2645(t112, t16506, t111, t5363, t1851, t3946, t1858, t3931, t1395, t5381, t1404, t6470);
        let (t55410, t55417) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2646(t1851, t2319, t4072, t12521, t12524, t12813, t1458, t16535, t19534, t20181, t2363, t3941, t45560, t5371, t5376, t5456, t5493, t55341, t55353, t55388, t671);
        let (t55420, t55457) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2647(t19530, t626, t12774, t12795, t12802, t1447, t16, t19488, t19489, t19492, t19499, t19503, t19504, t19517, t2219, t2248, t2336, t2341, t2351, t2355, t30171, t30307, t45697, t45707, t45751, t45762, t5469, t5472, t5475, t657, t659, t92);
        let t55512 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2648(t1447, t2349, t100, t12792, t12796, t12799, t12805, t19493, t19498, t19513, t19521, t19525, t21, t2248, t2341, t2342, t2350, t2354, t4049, t4059, t45460, t45496, t45717, t5396, t5468, t5480, t5484, t584, t662, t9, t92, t9384, t9398);
        let t55530 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2649(t4067, t2331, t45421, t45422, t45424, t45426, t45656, t45658, t45660, t45662, t45688, t45690, t55420, t55457, t55512, t64, t656);
        let t55566 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2650(t2281, t5489, t5465, t19474, t626, t19483, t19477, t12808, t19473, t19482, t19529, t2331, t2332, t2358, t26129, t29903, t4043, t4067, t45435, t45676, t5464, t5488, t64, t666, t9365);
        let (t55568, t55571, t55631) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2651(t109, t55530, t55566, t2363, t576, t12649, t12652, t12653, t12656, t12661, t12708, t1410, t1426, t1434, t19343, t19346, t19349, t19441, t2304, t3961, t3962, t3967, t3997, t4018, t5403, t609, t642, t80);
        let (t55662, t55666, t55673) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2652(t1409, t628, t67, t2250, t5398, t16558, t607, t12606, t12620, t12623, t12662, t12665, t1411, t1434, t17635, t1864, t19322, t19323, t19363, t19404, t2251, t3966, t3968, t3971, t4018, t5427, t608, t642, t65, t6509, t80);
    (t55368, t55374, t55376, t55378, t55410, t55417, t55568, t55571, t55631, t55662, t55666, t55673)
}
