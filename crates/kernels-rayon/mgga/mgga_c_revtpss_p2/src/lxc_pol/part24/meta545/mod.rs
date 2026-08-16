//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta545 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1612;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1613;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1614;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1615;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1616;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1617;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta545(t45: f64, t57: f64, t18367: f64, t22671: f64, t2299: f64, t4328: f64, t5825: f64, t766: f64, t80: f64, t87107: f64, t87126: f64, t87145: f64, t18379: f64, t2306: f64, t4335: f64, t770: f64, t83: f64, zeta_threshold: f64, t5962: f64, t5966: f64, t124: f64, t1544: f64, t1559: f64, t23266: f64, t2730: f64, t2745: f64, t2747: f64, t40507: f64, t40607: f64, t40611: f64, t40868: f64, t50436: f64, t50611: f64, t61677: f64, t61699: f64, t61797: f64, t61833: f64, t76279: f64, t76500: f64, t76502: f64, t76572: f64, t799: f64, t800: f64, t40638: f64, t40654: f64, t50703: f64, t61839: f64, t61877: f64, t61888: f64, t61890: f64, t61892: f64, t61924: f64, t76583: f64, t76587: f64, t76591: f64, t76593: f64, t76596: f64, t76615: f64, t76619: f64, t76645: f64, t76647: f64, t14894: f64, t18426: f64, t2477: f64, t40462: f64, t40737: f64, t40759: f64, t40771: f64, t4364: f64, t61981: f64, t76242: f64, t76672: f64, t76677: f64, t76689: f64, t76701: f64, t76703: f64, t76720: f64, t76738: f64, t76740: f64, t76764: f64, t828: f64, t851: f64, t855: f64, t2723: f64, t87399: f64, t39419: f64, t39422: f64, t39429: f64, t39432: f64, t87262: f64, t87263: f64, t87265: f64, t87267: f64, t87268: f64, t87296: f64, t87298: f64, t39442: f64, t39483: f64, t39520: f64, t87303: f64, t87304: f64, t87305: f64, t87306: f64, t87307: f64, t87309: f64, t87312: f64, t87314: f64, t39528: f64, t39531: f64, t39534: f64, t39537: f64, t39540: f64, t39741: f64, t39744: f64, t39747: f64, t39750: f64, t87315: f64, t87318: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t87529, t87541) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1612(t45, t57, t18367, t22671, t2299, t4328, t5825, t766, t80, t87107, t87126, t87145, t18379, t2306, t4335, t770, t83, zeta_threshold);
        let (t87543, t87548, t87553, t87562) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1613(t87529, t87541, t5962, t5966, t124, t1544, t1559, t23266, t2730, t2745, t2747, t40507, t40607, t40611, t40868, t50436, t50611, t61677, t61699, t61797, t61833, t76279, t76500, t76502, t76572, t799, t800);
        let t87579 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1614(t40638, t40654, t50703, t61839, t61877, t61888, t61890, t61892, t61924, t76583, t76587, t76591, t76593, t76596, t76615, t76619, t76645, t76647);
        let t87608 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1615(t14894, t18426, t2477, t40462, t40737, t40759, t40771, t4364, t61981, t76242, t76672, t76677, t76689, t76701, t76703, t76720, t76738, t76740, t76764, t828, t851, t855, t87543, t87548, t87553);
        let (t87629, t87634) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1616(t2723, t87399, t39419, t39422, t39429, t39432, t87262, t87263, t87265, t87267, t87268, t87296, t87298);
        let (t87635, t87637) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1617(t39442, t39483, t39520, t87303, t87304, t87305, t87306, t87307, t87309, t87312, t87314, t39528, t39531, t39534, t39537, t39540, t39741, t39744, t39747, t39750, t87315, t87318);
    (t87543, t87548, t87553, t87562, t87579, t87608, t87629, t87634, t87635, t87637)
}
