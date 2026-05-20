//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta380 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1378;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1379;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1380;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1381;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1382;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta380<F: Float>(t2452: F, t9720: F, t225: F, t268: F, t2665: F, t10868: F, t240: F, t10871: F, t2661: F, t40479: F, t10726: F, t2723: F, t10638: F, t231: F, t243: F, t2662: F, t10722: F, t2656: F, t2237: F, t2482: F, t849: F, t2677: F, t10489: F, t221: F, t2674: F, t2675: F, t234: F, t9801: F, t10887: F, t136: F, t2475: F, t220: F, t10777: F, t2731: F, t837: F, t2668: F, t823: F, t10782: F, t159: F, t33127: F, t64: F, t222: F, t10779: F, t2749: F, t40578: F, t10627: F, t10900: F, t125: F, t2430: F, t2745: F, t39476: F, t40673: F, t40679: F, t40681: F, t40686: F, t800: F, t828: F, t851: F, t855: F, t10794: F, t10811: F, t10807: F, t10709: F, t10760: F, t9794: F, t124: F, t138: F, t40649: F, t9645: F, t810: F, t10732: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t40688, t40690, t40691, t40696, t40700) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1378::<F>(t2452, t9720, t225, t268, t2665, t10868, t240, t10871, t2661, t40479, t10726, t2723);
        let (t40705, t40707, t40711, t40719) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1379::<F>(t10638, t231, t243, t2661, t2662, t10722, t2656, t2237, t2482, t849, t2677, t10489, t221, t2674, t2675);
        let (t40722, t40728, t40731) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1380::<F>(t234, t9801, t10887, t136, t2475, t220, t10777, t2731, t837, t2482, t2668, t823);
        let (t40735, t40746) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1381::<F>(t10782, t40731, t159, t33127, t64, t222, t10777, t10779, t2749, t40578, t10627, t10900, t125, t2430, t2731, t2745, t39476, t40673, t40679, t40681, t40686, t40691, t40696, t40700, t40705, t40707, t40711, t40719, t40722, t40728, t800, t828, t837, t851, t855);
        let (t40748, t40750, t40753, t40757, t40759, t40761) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1382::<F>(t10794, t10811, t10807, t10709, t10760, t9794, t124, t138, t40649, t9645, t810, t10732);
    (t40688, t40690, t40735, t40746, t40748, t40750, t40753, t40757, t40759, t40761)
}
