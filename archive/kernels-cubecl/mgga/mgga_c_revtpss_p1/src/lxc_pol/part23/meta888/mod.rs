//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta888 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2812;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2813;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2814;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2815;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2816;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2817;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2818;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta888<F: Float>(t10529: F, t2782: F, t76106: F, t233: F, t23359: F, t689: F, t869: F, t14598: F, t23160: F, t686: F, t72: F, t23244: F, t251: F, t1568: F, t5977: F, t2723: F, t4503: F, t1558: F, t6041: F, t231: F, t2783: F, t4500: F, t62967: F, t10661: F, t14972: F, t23172: F, t4366: F, t4504: F, t51299: F, t6017: F, t62606: F, t62609: F, t820: F, t23168: F, t39598: F, t10530: F, t6016: F, t2797: F, t23167: F, t62615: F, t62619: F, t62626: F, t62630: F, t62633: F, t62635: F, t62639: F, t40325: F, t836: F, t18616: F, t18681: F, t39624: F, t4424: F, t4514: F, t4526: F, t51320: F, t51355: F, t5978: F, t62644: F, t62649: F, t62651: F, t62653: F, t18719: F, t51549: F, t23245: F, t2798: F, t874: F, t10871: F, t14495: F, t14502: F, t14546: F, t14587: F, t18699: F, t39649: F, t40258: F, t4494: F, t51374: F, t62682: F, t837: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t76108, t76117, t76125, t76127) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2812::<F>(t10529, t2782, t76106, t233, t23359, t689, t869, t14598, t23160, t686, t72, t23244, t251);
        let t76131 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2813::<F>(t1568, t5977);
        let (t76136, t76147) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2814::<F>(t2723, t2782, t4503, t76131, t1558, t6041, t231, t2783, t4500, t62967, t10661, t14972, t23172, t4366, t4504, t51299, t6017, t62606, t62609, t76117, t76125, t76127, t820);
        let (t76153, t76158, t76163, t76169) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2815::<F>(t23168, t39598, t686, t72, t10530, t23172, t1558, t231, t6016, t2782, t2797, t23167, t251);
        let t76174 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2816::<F>(t231, t2782, t2783, t76169, t62615, t62619, t62626, t62630, t62633, t62635, t62639, t76153, t76158, t76163);
        let (t76194, t76198) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2817::<F>(t231, t2782, t2783, t76131, t40325, t836, t14972, t18616, t18681, t39624, t4424, t4514, t4526, t51320, t51355, t5978, t62644, t62649, t62651, t62653, t76169, t820);
        let (t76206, t76242, t76247) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2818::<F>(t18719, t51549, t23245, t2798, t686, t72, t23359, t874, t10871, t6016, t14495, t14502, t14546, t14587, t18699, t23160, t23168, t39649, t40258, t4494, t4504, t4514, t51374, t62682, t76131, t820, t836, t837);
    (t76108, t76127, t76131, t76136, t76147, t76169, t76174, t76194, t76198, t76206, t76242, t76247)
}
