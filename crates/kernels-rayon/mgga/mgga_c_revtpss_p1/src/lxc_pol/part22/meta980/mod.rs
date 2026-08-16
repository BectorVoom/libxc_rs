//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta980 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3302;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3303;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3304;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3305;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3306;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3307;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3308;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3309;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3310;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta980(t23160: f64, t836: f64, t10529: f64, t2782: f64, t14520: f64, t14606: f64, t39576: f64, t39581: f64, t39586: f64, t39595: f64, t51298: f64, t62577: f64, t62583: f64, t62587: f64, t62591: f64, t62595: f64, t62601: f64, t6016: f64, t860: f64, t231: f64, t2783: f64, t18657: f64, t686: f64, t72: f64, t874: f64, t1559: f64, t4423: f64, t2797: f64, t14586: f64, t18725: f64, t2470: f64, t2798: f64, t10542: f64, t18730: f64, t61749: f64, t18615: f64, t251: f64, t10069: f64, t18738: f64, t18742: f64, t10073: f64, t18677: f64, t18681: f64, t2724: f64, t4504: f64, t10530: f64, t18718: f64, t18719: f64, t39609: f64, t18761: f64, t14602: f64, t2482: f64, t2811: f64, t5977: f64, t213: f64, t234: f64, t39624: f64, t39633: f64, t39635: f64, t39640: f64, t51339: f64, t51355: f64, t51371: f64, t51373: f64, t62509: f64, t2801: f64, t879: f64, t18750: f64, t6041: f64, t61756: f64, t39649: f64, t39652: f64, t39662: f64, t39673: f64, t39678: f64, t39683: f64, t51390: f64, t51403: f64, t51408: f64, t61648: f64, t820: f64, t136: f64, t2457: f64, t2710: f64, t10535: f64, t5978: f64, t10943: f64, t14663: f64, t18699: f64, t18714: f64, t2754: f64, t39687: f64, t4494: f64, t4514: f64, t51418: f64, t51422: f64, t51424: f64, t51429: f64, t51434: f64, t51438: f64, t51442: f64, t10657: f64, t39694: f64, t39697: f64, t39701: f64, t39719: f64, t39723: f64, t39724: f64, t39726: f64, t4366: f64, t51320: f64, t51445: f64, t51452: f64, t51460: f64, t61679: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let t62611 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3302(t23160, t836, t10529, t2782, t14520, t14606, t39576, t39581, t39586, t39595, t51298, t62577, t62583, t62587, t62591, t62595, t62601);
        let (t62612, t62615, t62619, t62626) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3303(t6016, t860, t231, t2782, t2783, t18657, t686, t72, t874, t1559, t4423, t2797);
        let (t62630, t62633, t62635, t62639) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3304(t14586, t4423, t10529, t2782, t18725, t2470, t2798, t10542, t18730, t231, t61749, t2797);
        let (t62641, t62655) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3305(t18615, t251, t231, t2782, t2783, t10069, t18738, t18742, t10073, t18677, t18681, t2724, t4504, t62615, t62619, t62626, t62630, t62633, t62635, t62639);
        let t62679 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3306(t10530, t18718, t2470, t18719, t39609, t18761, t874, t14602, t2482, t2811, t5977, t213, t234, t39624, t39633, t39635, t39640, t51339, t51355, t51371, t51373, t62509);
        let (t62682, t62684, t62693, t62695) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3307(t2482, t2801, t5977, t879, t10073, t18750, t231, t2782, t2783, t6041, t836, t61756);
        let t62705 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3308(t2782, t2797, t62695, t39649, t39652, t39662, t39673, t39678, t39683, t51390, t51403, t51408, t61648, t62682, t62684, t62693, t820, t879);
        let t62733 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3309(t136, t2457, t2710, t6041, t10535, t5978, t10943, t14663, t18699, t18714, t2754, t39687, t4494, t4504, t4514, t51418, t51422, t51424, t51429, t51434, t51438, t51442, t820);
        let t62754 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3310(t10657, t18677, t39694, t39697, t39701, t39719, t39723, t39724, t39726, t4366, t4504, t51320, t51445, t51452, t51460, t5978, t61679, t62612, t62641, t820);
    (t62611, t62612, t62641, t62655, t62679, t62705, t62733, t62754)
}
