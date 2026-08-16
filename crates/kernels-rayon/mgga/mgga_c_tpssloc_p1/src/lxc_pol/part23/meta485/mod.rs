//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta485 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1484;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1485;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1486;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1487;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1488;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1489;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta485(t1410: f64, t1434: f64, t1864: f64, t19322: f64, t20207: f64, t20217: f64, t20222: f64, t20227: f64, t20264: f64, t20265: f64, t33: f64, t5398: f64, t5399: f64, t5400: f64, t5427: f64, t5442: f64, t65: f64, t7445: f64, t75361: f64, t75847: f64, t79692: f64, t80: f64, t12571: f64, t1437: f64, t19299: f64, t20201: f64, t20204: f64, t20288: f64, t2240: f64, t39030: f64, t39032: f64, t39034: f64, t39036: f64, t39038: f64, t39040: f64, t39043: f64, t39063: f64, t3953: f64, t45844: f64, t5389: f64, t5445: f64, t55921: f64, t605: f64, t75284: f64, t79579: f64, t79585: f64, t79637: f64, t86: f64, t9239: f64, t5: f64, t112: f64, t113: f64, t1442: f64, t1459: f64, t1774: f64, t1778: f64, t19451: f64, t20347: f64, t20698: f64, t20702: f64, t20717: f64, t22425: f64, t28002: f64, t4028: f64, t510: f64, t5450: f64, t5457: f64, t5494: f64, t6287: f64, t652: f64, t67001: f64, t7458: f64, t77944: f64, t79553: f64, t5464: f64, t5488: f64, t5468: f64, t5396: f64, t5480: f64, t5484: f64, t75910: f64, t100: f64, t103: f64, t104: f64, t1447: f64, t1450: f64, t19488: f64, t19513: f64, t20245: f64, t20318: f64, t20322: f64, t20332: f64, t20335: f64, t20338: f64, t20339: f64, t2341: f64, t2349: f64, t4049: f64, t4059: f64, t45460: f64, t45496: f64, t5475: f64, t5481: f64, t5485: f64, t92: f64, t95: f64, tau1: f64, t19473: f64, t20342: f64, t2331: f64, t4043: f64, t45421: f64, t45435: f64, t45656: f64, t55531: f64, t55537: f64, t64: f64, t656: f64, t75592: f64, t75601: f64, t75613: f64, t109: f64, t5493: f64, t5449: f64, t5456: f64, t53777: f64, t53779: f64, t56099: f64, t56104: f64, t73967: f64, t53798: f64, t1799: f64, t19596: f64, t20067: f64, t20675: f64, t28830: f64, t3918: f64, t39249: f64, t39256: f64, t39261: f64, t5160: f64, t5161: f64, t6347: f64, t74068: f64, t75240: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t79707 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1484(t1410, t1434, t1864, t19322, t20207, t20217, t20222, t20227, t20264, t20265, t33, t5398, t5399, t5400, t5427, t5442, t65, t7445, t75361, t75847, t79692, t80);
        let t79711 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1485(t12571, t1437, t19299, t20201, t20204, t20288, t2240, t39030, t39032, t39034, t39036, t39038, t39040, t39043, t39063, t3953, t45844, t5389, t5445, t55921, t605, t75284, t79579, t79585, t79637, t79707, t86, t9239);
        let (t79713, t79729) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1486(t5, t79711, t112, t113, t1442, t1459, t1774, t1778, t19451, t20347, t20698, t20702, t20717, t22425, t28002, t4028, t510, t5450, t5457, t5494, t6287, t652, t67001, t7458, t77944, t79553);
        let (t79748, t79755, t79812) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1487(t5464, t5488, t5468, t5396, t5480, t5484, t75910, t100, t103, t104, t1447, t1450, t19488, t19513, t20245, t20318, t20322, t20332, t20335, t20338, t20339, t2341, t2349, t4049, t4059, t45460, t45496, t5475, t5481, t5485, t92, t95, tau1);
        let t79816 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1488(t19473, t20342, t2331, t4043, t45421, t45435, t45656, t5488, t55531, t55537, t64, t656, t75592, t75601, t75613, t79748, t79755, t79812);
        let (t79817, t79825, t79829, t79834, t79835, t79836, t79837, t79853, t79854, t79855) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1489(t109, t79816, t5493, t5449, t5456, t53777, t53779, t56099, t56104, t73967, t53798, t1799, t19596, t20067, t20675, t28830, t3918, t39249, t39256, t39261, t5160, t5161, t6347, t74068, t75240);
    (t79713, t79729, t79817, t79825, t79829, t79834, t79835, t79836, t79837, t79853, t79854, t79855)
}
