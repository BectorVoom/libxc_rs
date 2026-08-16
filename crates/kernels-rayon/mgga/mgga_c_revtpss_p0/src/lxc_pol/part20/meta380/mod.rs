//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta380 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1378;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1379;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1380;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1381;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1382;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta380(t2452: f64, t9720: f64, t225: f64, t268: f64, t2665: f64, t10868: f64, t240: f64, t10871: f64, t2661: f64, t40479: f64, t10726: f64, t2723: f64, t10638: f64, t231: f64, t243: f64, t2662: f64, t10722: f64, t2656: f64, t2237: f64, t2482: f64, t849: f64, t2677: f64, t10489: f64, t221: f64, t2674: f64, t2675: f64, t234: f64, t9801: f64, t10887: f64, t136: f64, t2475: f64, t220: f64, t10777: f64, t2731: f64, t837: f64, t2668: f64, t823: f64, t10782: f64, t159: f64, t33127: f64, t64: f64, t222: f64, t10779: f64, t2749: f64, t40578: f64, t10627: f64, t10900: f64, t125: f64, t2430: f64, t2745: f64, t39476: f64, t40673: f64, t40679: f64, t40681: f64, t40686: f64, t800: f64, t828: f64, t851: f64, t855: f64, t10794: f64, t10811: f64, t10807: f64, t10709: f64, t10760: f64, t9794: f64, t124: f64, t138: f64, t40649: f64, t9645: f64, t810: f64, t10732: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40688, t40690, t40691, t40696, t40700) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1378(t2452, t9720, t225, t268, t2665, t10868, t240, t10871, t2661, t40479, t10726, t2723);
        let (t40705, t40707, t40711, t40719) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1379(t10638, t231, t243, t2661, t2662, t10722, t2656, t2237, t2482, t849, t2677, t10489, t221, t2674, t2675);
        let (t40722, t40728, t40731) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1380(t234, t9801, t10887, t136, t2475, t220, t10777, t2731, t837, t2482, t2668, t823);
        let (t40735, t40746) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1381(t10782, t40731, t159, t33127, t64, t222, t10777, t10779, t2749, t40578, t10627, t10900, t125, t2430, t2731, t2745, t39476, t40673, t40679, t40681, t40686, t40691, t40696, t40700, t40705, t40707, t40711, t40719, t40722, t40728, t800, t828, t837, t851, t855);
        let (t40748, t40750, t40753, t40757, t40759, t40761) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1382(t10794, t10811, t10807, t10709, t10760, t9794, t124, t138, t40649, t9645, t810, t10732);
    (t40688, t40690, t40735, t40746, t40748, t40750, t40753, t40757, t40759, t40761)
}
