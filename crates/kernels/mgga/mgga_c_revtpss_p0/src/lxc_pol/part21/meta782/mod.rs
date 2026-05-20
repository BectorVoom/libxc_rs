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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta782<F: Float>(t14524: F, t39575: F, t10867: F, t1568: F, t14939: F, t233: F, t689: F, t869: F, t10069: F, t14588: F, t10518: F, t14606: F, t10872: F, t40298: F, t40303: F, t40307: F, t40311: F, t40314: F, t40316: F, t40318: F, t820: F, t231: F, t2782: F, t2783: F, t51380: F, t10073: F, t14504: F, t10547: F, t14568: F, t50560: F, t2797: F, t18632: F, t836: F, t10529: F, t14602: F, t2482: F, t2811: F, t4423: F, t14575: F, t2435: F, t10943: F, t14598: F, t686: F, t72: F, t10541: F, t14495: F, t4503: F, t786: F, t10532: F, t40270: F, t4496: F, t136: F, t137: F, t14597: F, t2438: F, t2723: F, t49180: F, t2457: F, t2710: F, t4469: F, t2722: F, t50474: F, t39597: F, t14586: F, t10115: F, t1576: F, t14593: F, t2470: F, t874: F, t10861: F, t14502: F, t14535: F, t14961: F, t2754: F, t40894: F, t4504: F, t4514: F, t1558: F, t2801: F, t2815: F, t10538: F, t14605: F, t2645: F) -> (F, F, F, F, F, F, F, F) {
        let (t51484, t51498, t51505, t51507, t51512) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2800::<F>(t14524, t39575, t10867, t1568, t14939, t233, t689, t869, t10069, t14588, t10518, t14606);
        let t51515 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2801::<F>(t51512, t10872, t40298, t40303, t40307, t40311, t40314, t40316, t40318, t51498, t51505, t51507, t820);
        let (t51519, t51522, t51523, t51527, t51529) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2802::<F>(t231, t2782, t2783, t51380, t10073, t14504, t10547, t14568, t50560, t2797, t18632, t836);
        let (t51531, t51535, t51538, t51541) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2803::<F>(t10529, t2782, t51529, t14602, t2482, t2811, t4423, t14575, t2435, t10943, t14598, t686, t72);
        let t51552 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2804::<F>(t10541, t14495, t2782, t10518, t14568, t1568, t4503, t786, t10532, t51519, t51522, t51523, t51527, t51531, t51535, t51538, t51541);
        let (t51553, t51561, t51564) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2805::<F>(t40270, t4496, t136, t137, t14597, t2438, t2723, t49180, t836, t2457, t2710, t4469);
        let (t51565, t51572, t51576, t51578, t51587) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2806::<F>(t51564, t2722, t50474, t2782, t39597, t14586, t10529, t10115, t1576, t14593, t2470, t874);
        let t51589 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2807::<F>(t51587, t10861, t10943, t14502, t14535, t14961, t2754, t40894, t4504, t4514, t51553, t51561, t51565, t51572, t51576, t51578, t820);
        let (t51598, t51600, t51604, t51610) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2808::<F>(t1558, t2482, t2801, t2815, t10547, t14606, t10538, t14605, t49180, t14586, t2645, t10529, t2782);
    (t51484, t51515, t51552, t51589, t51598, t51600, t51604, t51610)
}
