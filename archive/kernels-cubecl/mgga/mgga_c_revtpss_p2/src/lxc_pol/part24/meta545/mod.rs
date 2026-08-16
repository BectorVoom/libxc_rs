//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta545 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1612;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1613;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1614;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1615;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1616;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1617;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta545<F: Float>(t45: F, t57: F, t18367: F, t22671: F, t2299: F, t4328: F, t5825: F, t766: F, t80: F, t87107: F, t87126: F, t87145: F, t18379: F, t2306: F, t4335: F, t770: F, t83: F, zeta_threshold: F, t5962: F, t5966: F, t124: F, t1544: F, t1559: F, t23266: F, t2730: F, t2745: F, t2747: F, t40507: F, t40607: F, t40611: F, t40868: F, t50436: F, t50611: F, t61677: F, t61699: F, t61797: F, t61833: F, t76279: F, t76500: F, t76502: F, t76572: F, t799: F, t800: F, t40638: F, t40654: F, t50703: F, t61839: F, t61877: F, t61888: F, t61890: F, t61892: F, t61924: F, t76583: F, t76587: F, t76591: F, t76593: F, t76596: F, t76615: F, t76619: F, t76645: F, t76647: F, t14894: F, t18426: F, t2477: F, t40462: F, t40737: F, t40759: F, t40771: F, t4364: F, t61981: F, t76242: F, t76672: F, t76677: F, t76689: F, t76701: F, t76703: F, t76720: F, t76738: F, t76740: F, t76764: F, t828: F, t851: F, t855: F, t2723: F, t87399: F, t39419: F, t39422: F, t39429: F, t39432: F, t87262: F, t87263: F, t87265: F, t87267: F, t87268: F, t87296: F, t87298: F, t39442: F, t39483: F, t39520: F, t87303: F, t87304: F, t87305: F, t87306: F, t87307: F, t87309: F, t87312: F, t87314: F, t39528: F, t39531: F, t39534: F, t39537: F, t39540: F, t39741: F, t39744: F, t39747: F, t39750: F, t87315: F, t87318: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t87529, t87541) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1612::<F>(t45, t57, t18367, t22671, t2299, t4328, t5825, t766, t80, t87107, t87126, t87145, t18379, t2306, t4335, t770, t83, zeta_threshold);
        let (t87543, t87548, t87553, t87562) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1613::<F>(t87529, t87541, t5962, t5966, t124, t1544, t1559, t23266, t2730, t2745, t2747, t40507, t40607, t40611, t40868, t50436, t50611, t61677, t61699, t61797, t61833, t76279, t76500, t76502, t76572, t799, t800);
        let t87579 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1614::<F>(t40638, t40654, t50703, t61839, t61877, t61888, t61890, t61892, t61924, t76583, t76587, t76591, t76593, t76596, t76615, t76619, t76645, t76647);
        let t87608 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1615::<F>(t14894, t18426, t2477, t40462, t40737, t40759, t40771, t4364, t61981, t76242, t76672, t76677, t76689, t76701, t76703, t76720, t76738, t76740, t76764, t828, t851, t855, t87543, t87548, t87553);
        let (t87629, t87634) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1616::<F>(t2723, t87399, t39419, t39422, t39429, t39432, t87262, t87263, t87265, t87267, t87268, t87296, t87298);
        let (t87635, t87637) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1617::<F>(t39442, t39483, t39520, t87303, t87304, t87305, t87306, t87307, t87309, t87312, t87314, t39528, t39531, t39534, t39537, t39540, t39741, t39744, t39747, t39750, t87315, t87318);
    (t87543, t87548, t87553, t87562, t87579, t87608, t87629, t87634, t87635, t87637)
}
