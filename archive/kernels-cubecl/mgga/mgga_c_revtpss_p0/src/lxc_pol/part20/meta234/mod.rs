//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta234 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1032;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1033;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1034;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1035;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta234<F: Float>(t10631: F, t808: F, t10886: F, t2699: F, t798: F, t802: F, t2703: F, t2707: F, t10489: F, t124: F, t800: F, t159: F, t853: F, t216: F, t10627: F, t2729: F, t794: F, t2732: F, t10853: F, t10855: F, t10859: F, t10863: F, t10870: F, t10874: F, t10878: F, t10881: F, t10885: F, t2721: F, t799: F, t825: F, t10725: F, t10791: F, t10848: F, t136: F, t860: F, t2457: F, t2710: F, t10519: F, t10524: F, t10533: F, t10539: F, t10543: F, t10548: F, t10639: F, t10645: F, t10647: F, t10651: F, t10655: F, t10657: F, t10661: F, t10666: F, t213: F, t234: F, t2646: F, t2724: F, t2815: F, t820: F, t837: F, t879: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t10887, t10888, t10890, t10891, t10893, t10895, t10896, t10899) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1032::<F>(t10631, t808, t10886, t2699, t798, t802, t2703, t2707, t10489, t124, t800, t159, t853);
        let (t10900, t10902, t10905, t10908) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1033::<F>(t10899, t216, t10627, t124, t800, t2729, t794, t2732, t10853, t10855, t10859, t10863, t10870, t10874, t10878, t10881, t10885, t10888, t10891, t10893, t10896, t2721, t799, t825);
        let t10910 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1034::<F>(t10725, t10791, t10848, t10908);
        let (t10914, t10918) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1035::<F>(t136, t860, t2457, t2710, t10519, t10524, t10533, t10539, t10543, t10548, t10639, t10645, t10647, t10651, t10655, t10657, t10661, t10666, t10910, t213, t234, t2646, t2724, t2815, t820, t837, t879);
    (t10887, t10890, t10895, t10896, t10899, t10900, t10902, t10905, t10910, t10914, t10918)
}
