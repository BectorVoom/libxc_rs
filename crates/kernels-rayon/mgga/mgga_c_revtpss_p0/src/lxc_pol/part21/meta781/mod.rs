//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta781 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2793;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2794;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2795;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2796;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2797;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2798;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2799;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta781(t14939: f64, t822: f64, t686: f64, t72: f64, t874: f64, t14574: f64, t2439: f64, t2777: f64, t10943: f64, t14502: f64, t14507: f64, t14546: f64, t14547: f64, t2724: f64, t2754: f64, t39588: f64, t39620: f64, t39624: f64, t39629: f64, t4494: f64, t4504: f64, t4514: f64, t820: f64, t837: f64, t40297: f64, t4500: f64, t10069: f64, t14504: f64, t4423: f64, t860: f64, t1558: f64, t2760: f64, t10639: f64, t10666: f64, t14535: f64, t14663: f64, t2646: f64, t2815: f64, t39633: f64, t39635: f64, t39640: f64, t4366: f64, t4526: f64, t14557: f64, t9303: f64, t2718: f64, t4469: f64, t4519: f64, t9292: f64, t2798: f64, t4499: f64, t9288: f64, t10542: f64, t14520: f64, t2783: f64, t786: f64, t2801: f64, t10073: f64, t14588: f64, t14563: f64, t39683: f64, t39685: f64, t39687: f64, t14519: f64, t2470: f64, t231: f64, t51049: f64, t2782: f64, t2797: f64, t4522: f64, t1573: f64, t40317: f64, t39692: f64, t39694: f64, t39697: f64, t39701: f64, t39707: f64, t14587: f64, t39608: f64, t14496: f64, t10657: f64, t39712: f64, t39719: f64, t39723: f64, t39724: f64, t39726: f64, t40284: f64, t4424: f64, t836: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t51360 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2793(t14939, t822, t686, t72, t874, t14574, t2439, t2777, t10943, t14502, t14507, t14546, t14547, t2724, t2754, t39588, t39620, t39624, t39629, t4494, t4504, t4514, t820, t837);
        let (t51375, t51380, t51387) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2794(t40297, t4500, t10069, t14504, t4423, t860, t1558, t2760, t10639, t10666, t14535, t14663, t2646, t2815, t39633, t39635, t39640, t4366, t4504, t4514, t4526, t820);
        let (t51390, t51396, t51403, t51408, t51418, t51421) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2795(t14557, t9303, t2718, t4469, t4519, t9292, t2798, t4499, t9288, t10542, t14520, t2783, t786);
        let t51431 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2796(t2801, t51421, t10073, t14588, t10542, t14563, t14502, t14507, t14546, t14547, t2646, t2724, t39683, t39685, t39687, t4504, t4514, t51408, t51418);
        let (t51435, t51438, t51442, t51445) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2797(t14519, t2470, t2798, t231, t51049, t2782, t2797, t14663, t686, t72, t4522, t874, t9288);
        let t51456 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2798(t1573, t40317, t39692, t39694, t39697, t39701, t39707, t4514, t51380, t51435, t51438, t51442, t51445, t837);
        let t51479 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2799(t14587, t2782, t39608, t10069, t14496, t10639, t10657, t14546, t39712, t39719, t39723, t39724, t39726, t40284, t4424, t4494, t4514, t51375, t820, t836, t837);
    (t51360, t51375, t51380, t51387, t51390, t51396, t51403, t51431, t51456, t51479)
}
