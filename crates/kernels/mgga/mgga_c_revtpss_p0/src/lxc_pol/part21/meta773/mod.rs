//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta773 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2745;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2746;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2747;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2748;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2749;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta773<F: Float>(t50504: F, t1558: F, t2722: F, t10726: F, t2661: F, t2724: F, t4416: F, t4352: F, t10722: F, t4435: F, t14751: F, t2652: F, t14468: F, t14791: F, t2477: F, t2745: F, t2749: F, t40455: F, t40471: F, t40473: F, t40475: F, t40477: F, t40482: F, t40484: F, t40489: F, t50493: F, t50497: F, t50502: F, t775: F, t828: F, t851: F, t14769: F, t10716: F, t14757: F, t14772: F, t221: F, t2674: F, t40683: F, t10698: F, t14494: F, t14785: F, t14917: F, t2394: F, t40503: F, t40507: F, t40509: F, t40511: F, t40518: F, t40523: F, t40526: F, t40529: F, t40532: F, t40535: F, t40549: F, t40553: F, t40558: F, t4343: F, t2645: F, t10868: F, t2482: F, t814: F, t14547: F, t14671: F, t14686: F, t2662: F, t2754: F, t14738: F, t2741: F, t10845: F, t14732: F, t4423: F, t853: F, t14718: F, t14872: F, t10777: F, t10779: F, t1548: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t50505, t50511, t50518, t50522, t50524, t50526) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2745::<F>(t50504, t1558, t2722, t10726, t2661, t2724, t4416, t4352, t10722, t4435, t14751, t2652);
        let t50528 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2746::<F>(t14468, t14791, t2477, t2745, t2749, t40455, t40471, t40473, t40475, t40477, t40482, t40484, t40489, t50493, t50497, t50502, t50505, t50511, t50518, t50522, t50524, t50526, t775, t828, t851);
        let t50558 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2747::<F>(t14769, t2652, t10716, t14757, t14772, t221, t2674, t40683, t10698, t14494, t14785, t14917, t2394, t2745, t40503, t40507, t40509, t40511, t40518, t40523, t40526, t40529, t40532, t40535, t40549, t40553, t40558, t4343, t828, t851);
        let (t50560, t50573, t50577) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2748::<F>(t1558, t2645, t10868, t2482, t814, t14547, t14671, t14686, t2661, t2662, t2754, t4416);
        let (t50579, t50582, t50586, t50590, t50594) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2749::<F>(t14738, t2741, t10845, t14732, t4423, t853, t2661, t2662, t2749, t14718, t14872, t10777, t10779, t1548, t2754);
    (t50511, t50528, t50558, t50560, t50573, t50577, t50579, t50582, t50586, t50590, t50594)
}
