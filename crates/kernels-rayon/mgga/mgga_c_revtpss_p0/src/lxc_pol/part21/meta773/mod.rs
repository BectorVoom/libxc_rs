//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta773 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2745;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2746;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2747;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2748;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2749;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta773(t50504: f64, t1558: f64, t2722: f64, t10726: f64, t2661: f64, t2724: f64, t4416: f64, t4352: f64, t10722: f64, t4435: f64, t14751: f64, t2652: f64, t14468: f64, t14791: f64, t2477: f64, t2745: f64, t2749: f64, t40455: f64, t40471: f64, t40473: f64, t40475: f64, t40477: f64, t40482: f64, t40484: f64, t40489: f64, t50493: f64, t50497: f64, t50502: f64, t775: f64, t828: f64, t851: f64, t14769: f64, t10716: f64, t14757: f64, t14772: f64, t221: f64, t2674: f64, t40683: f64, t10698: f64, t14494: f64, t14785: f64, t14917: f64, t2394: f64, t40503: f64, t40507: f64, t40509: f64, t40511: f64, t40518: f64, t40523: f64, t40526: f64, t40529: f64, t40532: f64, t40535: f64, t40549: f64, t40553: f64, t40558: f64, t4343: f64, t2645: f64, t10868: f64, t2482: f64, t814: f64, t14547: f64, t14671: f64, t14686: f64, t2662: f64, t2754: f64, t14738: f64, t2741: f64, t10845: f64, t14732: f64, t4423: f64, t853: f64, t14718: f64, t14872: f64, t10777: f64, t10779: f64, t1548: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50505, t50511, t50518, t50522, t50524, t50526) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2745(t50504, t1558, t2722, t10726, t2661, t2724, t4416, t4352, t10722, t4435, t14751, t2652);
        let t50528 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2746(t14468, t14791, t2477, t2745, t2749, t40455, t40471, t40473, t40475, t40477, t40482, t40484, t40489, t50493, t50497, t50502, t50505, t50511, t50518, t50522, t50524, t50526, t775, t828, t851);
        let t50558 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2747(t14769, t2652, t10716, t14757, t14772, t221, t2674, t40683, t10698, t14494, t14785, t14917, t2394, t2745, t40503, t40507, t40509, t40511, t40518, t40523, t40526, t40529, t40532, t40535, t40549, t40553, t40558, t4343, t828, t851);
        let (t50560, t50573, t50577) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2748(t1558, t2645, t10868, t2482, t814, t14547, t14671, t14686, t2661, t2662, t2754, t4416);
        let (t50579, t50582, t50586, t50590, t50594) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2749(t14738, t2741, t10845, t14732, t4423, t853, t2661, t2662, t2749, t14718, t14872, t10777, t10779, t1548, t2754);
    (t50511, t50528, t50558, t50560, t50573, t50577, t50579, t50582, t50586, t50590, t50594)
}
