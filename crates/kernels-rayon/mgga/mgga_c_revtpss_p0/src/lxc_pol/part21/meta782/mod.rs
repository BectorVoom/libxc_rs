//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta782 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2800;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2801;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2802;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2803;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2804;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2805;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2806;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2807;
use chunk8::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2808;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta782(t14524: f64, t39575: f64, t10867: f64, t1568: f64, t14939: f64, t233: f64, t689: f64, t869: f64, t10069: f64, t14588: f64, t10518: f64, t14606: f64, t10872: f64, t40298: f64, t40303: f64, t40307: f64, t40311: f64, t40314: f64, t40316: f64, t40318: f64, t820: f64, t231: f64, t2782: f64, t2783: f64, t51380: f64, t10073: f64, t14504: f64, t10547: f64, t14568: f64, t50560: f64, t2797: f64, t18632: f64, t836: f64, t10529: f64, t14602: f64, t2482: f64, t2811: f64, t4423: f64, t14575: f64, t2435: f64, t10943: f64, t14598: f64, t686: f64, t72: f64, t10541: f64, t14495: f64, t4503: f64, t786: f64, t10532: f64, t40270: f64, t4496: f64, t136: f64, t137: f64, t14597: f64, t2438: f64, t2723: f64, t49180: f64, t2457: f64, t2710: f64, t4469: f64, t2722: f64, t50474: f64, t39597: f64, t14586: f64, t10115: f64, t1576: f64, t14593: f64, t2470: f64, t874: f64, t10861: f64, t14502: f64, t14535: f64, t14961: f64, t2754: f64, t40894: f64, t4504: f64, t4514: f64, t1558: f64, t2801: f64, t2815: f64, t10538: f64, t14605: f64, t2645: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t51484, t51498, t51505, t51507, t51512) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2800(t14524, t39575, t10867, t1568, t14939, t233, t689, t869, t10069, t14588, t10518, t14606);
        let t51515 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2801(t51512, t10872, t40298, t40303, t40307, t40311, t40314, t40316, t40318, t51498, t51505, t51507, t820);
        let (t51519, t51522, t51523, t51527, t51529) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2802(t231, t2782, t2783, t51380, t10073, t14504, t10547, t14568, t50560, t2797, t18632, t836);
        let (t51531, t51535, t51538, t51541) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2803(t10529, t2782, t51529, t14602, t2482, t2811, t4423, t14575, t2435, t10943, t14598, t686, t72);
        let t51552 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2804(t10541, t14495, t2782, t10518, t14568, t1568, t4503, t786, t10532, t51519, t51522, t51523, t51527, t51531, t51535, t51538, t51541);
        let (t51553, t51561, t51564) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2805(t40270, t4496, t136, t137, t14597, t2438, t2723, t49180, t836, t2457, t2710, t4469);
        let (t51565, t51572, t51576, t51578, t51587) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2806(t51564, t2722, t50474, t2782, t39597, t14586, t10529, t10115, t1576, t14593, t2470, t874);
        let t51589 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2807(t51587, t10861, t10943, t14502, t14535, t14961, t2754, t40894, t4504, t4514, t51553, t51561, t51565, t51572, t51576, t51578, t820);
        let (t51598, t51600, t51604, t51610) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2808(t1558, t2482, t2801, t2815, t10547, t14606, t10538, t14605, t49180, t14586, t2645, t10529, t2782);
    (t51484, t51515, t51552, t51589, t51598, t51600, t51604, t51610)
}
