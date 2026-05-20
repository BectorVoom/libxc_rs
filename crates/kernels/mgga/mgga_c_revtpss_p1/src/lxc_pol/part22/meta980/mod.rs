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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta980<F: Float>(t23160: F, t836: F, t10529: F, t2782: F, t14520: F, t14606: F, t39576: F, t39581: F, t39586: F, t39595: F, t51298: F, t62577: F, t62583: F, t62587: F, t62591: F, t62595: F, t62601: F, t6016: F, t860: F, t231: F, t2783: F, t18657: F, t686: F, t72: F, t874: F, t1559: F, t4423: F, t2797: F, t14586: F, t18725: F, t2470: F, t2798: F, t10542: F, t18730: F, t61749: F, t18615: F, t251: F, t10069: F, t18738: F, t18742: F, t10073: F, t18677: F, t18681: F, t2724: F, t4504: F, t10530: F, t18718: F, t18719: F, t39609: F, t18761: F, t14602: F, t2482: F, t2811: F, t5977: F, t213: F, t234: F, t39624: F, t39633: F, t39635: F, t39640: F, t51339: F, t51355: F, t51371: F, t51373: F, t62509: F, t2801: F, t879: F, t18750: F, t6041: F, t61756: F, t39649: F, t39652: F, t39662: F, t39673: F, t39678: F, t39683: F, t51390: F, t51403: F, t51408: F, t61648: F, t820: F, t136: F, t2457: F, t2710: F, t10535: F, t5978: F, t10943: F, t14663: F, t18699: F, t18714: F, t2754: F, t39687: F, t4494: F, t4514: F, t51418: F, t51422: F, t51424: F, t51429: F, t51434: F, t51438: F, t51442: F, t10657: F, t39694: F, t39697: F, t39701: F, t39719: F, t39723: F, t39724: F, t39726: F, t4366: F, t51320: F, t51445: F, t51452: F, t51460: F, t61679: F) -> (F, F, F, F, F, F, F, F) {
        let t62611 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3302::<F>(t23160, t836, t10529, t2782, t14520, t14606, t39576, t39581, t39586, t39595, t51298, t62577, t62583, t62587, t62591, t62595, t62601);
        let (t62612, t62615, t62619, t62626) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3303::<F>(t6016, t860, t231, t2782, t2783, t18657, t686, t72, t874, t1559, t4423, t2797);
        let (t62630, t62633, t62635, t62639) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3304::<F>(t14586, t4423, t10529, t2782, t18725, t2470, t2798, t10542, t18730, t231, t61749, t2797);
        let (t62641, t62655) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3305::<F>(t18615, t251, t231, t2782, t2783, t10069, t18738, t18742, t10073, t18677, t18681, t2724, t4504, t62615, t62619, t62626, t62630, t62633, t62635, t62639);
        let t62679 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3306::<F>(t10530, t18718, t2470, t18719, t39609, t18761, t874, t14602, t2482, t2811, t5977, t213, t234, t39624, t39633, t39635, t39640, t51339, t51355, t51371, t51373, t62509);
        let (t62682, t62684, t62693, t62695) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3307::<F>(t2482, t2801, t5977, t879, t10073, t18750, t231, t2782, t2783, t6041, t836, t61756);
        let t62705 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3308::<F>(t2782, t2797, t62695, t39649, t39652, t39662, t39673, t39678, t39683, t51390, t51403, t51408, t61648, t62682, t62684, t62693, t820, t879);
        let t62733 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3309::<F>(t136, t2457, t2710, t6041, t10535, t5978, t10943, t14663, t18699, t18714, t2754, t39687, t4494, t4504, t4514, t51418, t51422, t51424, t51429, t51434, t51438, t51442, t820);
        let t62754 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3310::<F>(t10657, t18677, t39694, t39697, t39701, t39719, t39723, t39724, t39726, t4366, t4504, t51320, t51445, t51452, t51460, t5978, t61679, t62612, t62641, t820);
    (t62611, t62612, t62641, t62655, t62679, t62705, t62733, t62754)
}
