//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta888 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2812;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2813;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2814;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2815;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2816;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2817;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2818;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta888(t10529: f64, t2782: f64, t76106: f64, t233: f64, t23359: f64, t689: f64, t869: f64, t14598: f64, t23160: f64, t686: f64, t72: f64, t23244: f64, t251: f64, t1568: f64, t5977: f64, t2723: f64, t4503: f64, t1558: f64, t6041: f64, t231: f64, t2783: f64, t4500: f64, t62967: f64, t10661: f64, t14972: f64, t23172: f64, t4366: f64, t4504: f64, t51299: f64, t6017: f64, t62606: f64, t62609: f64, t820: f64, t23168: f64, t39598: f64, t10530: f64, t6016: f64, t2797: f64, t23167: f64, t62615: f64, t62619: f64, t62626: f64, t62630: f64, t62633: f64, t62635: f64, t62639: f64, t40325: f64, t836: f64, t18616: f64, t18681: f64, t39624: f64, t4424: f64, t4514: f64, t4526: f64, t51320: f64, t51355: f64, t5978: f64, t62644: f64, t62649: f64, t62651: f64, t62653: f64, t18719: f64, t51549: f64, t23245: f64, t2798: f64, t874: f64, t10871: f64, t14495: f64, t14502: f64, t14546: f64, t14587: f64, t18699: f64, t39649: f64, t40258: f64, t4494: f64, t51374: f64, t62682: f64, t837: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t76108, t76117, t76125, t76127) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2812(t10529, t2782, t76106, t233, t23359, t689, t869, t14598, t23160, t686, t72, t23244, t251);
        let t76131 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2813(t1568, t5977);
        let (t76136, t76147) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2814(t2723, t2782, t4503, t76131, t1558, t6041, t231, t2783, t4500, t62967, t10661, t14972, t23172, t4366, t4504, t51299, t6017, t62606, t62609, t76117, t76125, t76127, t820);
        let (t76153, t76158, t76163, t76169) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2815(t23168, t39598, t686, t72, t10530, t23172, t1558, t231, t6016, t2782, t2797, t23167, t251);
        let t76174 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2816(t231, t2782, t2783, t76169, t62615, t62619, t62626, t62630, t62633, t62635, t62639, t76153, t76158, t76163);
        let (t76194, t76198) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2817(t231, t2782, t2783, t76131, t40325, t836, t14972, t18616, t18681, t39624, t4424, t4514, t4526, t51320, t51355, t5978, t62644, t62649, t62651, t62653, t76169, t820);
        let (t76206, t76242, t76247) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2818(t18719, t51549, t23245, t2798, t686, t72, t23359, t874, t10871, t6016, t14495, t14502, t14546, t14587, t18699, t23160, t23168, t39649, t40258, t4494, t4504, t4514, t51374, t62682, t76131, t820, t836, t837);
    (t76108, t76127, t76131, t76136, t76147, t76169, t76174, t76194, t76198, t76206, t76242, t76247)
}
