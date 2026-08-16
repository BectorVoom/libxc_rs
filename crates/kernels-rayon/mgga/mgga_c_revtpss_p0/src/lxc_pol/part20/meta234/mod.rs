//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta234 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1032;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1033;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1034;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1035;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta234(t10631: f64, t808: f64, t10886: f64, t2699: f64, t798: f64, t802: f64, t2703: f64, t2707: f64, t10489: f64, t124: f64, t800: f64, t159: f64, t853: f64, t216: f64, t10627: f64, t2729: f64, t794: f64, t2732: f64, t10853: f64, t10855: f64, t10859: f64, t10863: f64, t10870: f64, t10874: f64, t10878: f64, t10881: f64, t10885: f64, t2721: f64, t799: f64, t825: f64, t10725: f64, t10791: f64, t10848: f64, t136: f64, t860: f64, t2457: f64, t2710: f64, t10519: f64, t10524: f64, t10533: f64, t10539: f64, t10543: f64, t10548: f64, t10639: f64, t10645: f64, t10647: f64, t10651: f64, t10655: f64, t10657: f64, t10661: f64, t10666: f64, t213: f64, t234: f64, t2646: f64, t2724: f64, t2815: f64, t820: f64, t837: f64, t879: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10887, t10888, t10890, t10891, t10893, t10895, t10896, t10899) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1032(t10631, t808, t10886, t2699, t798, t802, t2703, t2707, t10489, t124, t800, t159, t853);
        let (t10900, t10902, t10905, t10908) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1033(t10899, t216, t10627, t124, t800, t2729, t794, t2732, t10853, t10855, t10859, t10863, t10870, t10874, t10878, t10881, t10885, t10888, t10891, t10893, t10896, t2721, t799, t825);
        let t10910 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1034(t10725, t10791, t10848, t10908);
        let (t10914, t10918) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1035(t136, t860, t2457, t2710, t10519, t10524, t10533, t10539, t10543, t10548, t10639, t10645, t10647, t10651, t10655, t10657, t10661, t10666, t10910, t213, t234, t2646, t2724, t2815, t820, t837, t879);
    (t10887, t10890, t10895, t10896, t10899, t10900, t10902, t10905, t10910, t10914, t10918)
}
