//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta781 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2793;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2794;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2795;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2796;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2797;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2798;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2799;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta781<F: Float>(t14939: F, t822: F, t686: F, t72: F, t874: F, t14574: F, t2439: F, t2777: F, t10943: F, t14502: F, t14507: F, t14546: F, t14547: F, t2724: F, t2754: F, t39588: F, t39620: F, t39624: F, t39629: F, t4494: F, t4504: F, t4514: F, t820: F, t837: F, t40297: F, t4500: F, t10069: F, t14504: F, t4423: F, t860: F, t1558: F, t2760: F, t10639: F, t10666: F, t14535: F, t14663: F, t2646: F, t2815: F, t39633: F, t39635: F, t39640: F, t4366: F, t4526: F, t14557: F, t9303: F, t2718: F, t4469: F, t4519: F, t9292: F, t2798: F, t4499: F, t9288: F, t10542: F, t14520: F, t2783: F, t786: F, t2801: F, t10073: F, t14588: F, t14563: F, t39683: F, t39685: F, t39687: F, t14519: F, t2470: F, t231: F, t51049: F, t2782: F, t2797: F, t4522: F, t1573: F, t40317: F, t39692: F, t39694: F, t39697: F, t39701: F, t39707: F, t14587: F, t39608: F, t14496: F, t10657: F, t39712: F, t39719: F, t39723: F, t39724: F, t39726: F, t40284: F, t4424: F, t836: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t51360 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2793::<F>(t14939, t822, t686, t72, t874, t14574, t2439, t2777, t10943, t14502, t14507, t14546, t14547, t2724, t2754, t39588, t39620, t39624, t39629, t4494, t4504, t4514, t820, t837);
        let (t51375, t51380, t51387) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2794::<F>(t40297, t4500, t10069, t14504, t4423, t860, t1558, t2760, t10639, t10666, t14535, t14663, t2646, t2815, t39633, t39635, t39640, t4366, t4504, t4514, t4526, t820);
        let (t51390, t51396, t51403, t51408, t51418, t51421) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2795::<F>(t14557, t9303, t2718, t4469, t4519, t9292, t2798, t4499, t9288, t10542, t14520, t2783, t786);
        let t51431 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2796::<F>(t2801, t51421, t10073, t14588, t10542, t14563, t14502, t14507, t14546, t14547, t2646, t2724, t39683, t39685, t39687, t4504, t4514, t51408, t51418);
        let (t51435, t51438, t51442, t51445) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2797::<F>(t14519, t2470, t2798, t231, t51049, t2782, t2797, t14663, t686, t72, t4522, t874, t9288);
        let t51456 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2798::<F>(t1573, t40317, t39692, t39694, t39697, t39701, t39707, t4514, t51380, t51435, t51438, t51442, t51445, t837);
        let t51479 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2799::<F>(t14587, t2782, t39608, t10069, t14496, t10639, t10657, t14546, t39712, t39719, t39723, t39724, t39726, t40284, t4424, t4494, t4514, t51375, t820, t836, t837);
    (t51360, t51375, t51380, t51387, t51390, t51396, t51403, t51431, t51456, t51479)
}
