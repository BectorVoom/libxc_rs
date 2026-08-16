//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta314 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1584;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1585;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1586;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1587;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta314(t10631: f64, t808: f64, t10886: f64, t2699: f64, t798: f64, t802: f64, t2703: f64, t2707: f64, t10489: f64, t124: f64, t800: f64, t159: f64, t853: f64, t216: f64, t10627: f64, t2729: f64, t794: f64, t2732: f64, t10853: f64, t10855: f64, t10859: f64, t10863: f64, t10870: f64, t10874: f64, t10878: f64, t10881: f64, t10885: f64, t2721: f64, t799: f64, t825: f64, t10725: f64, t10791: f64, t10848: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10887, t10888, t10890, t10891, t10893, t10896, t10899) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1584(t10631, t808, t10886, t2699, t798, t802, t2703, t2707, t10489, t124, t800, t159, t853);
        let (t10900, t10902, t10905) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1585(t10899, t216, t10627, t124, t800, t2729, t794);
        let (t10906, t10908) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1586(t10905, t2732, t10853, t10855, t10859, t10863, t10870, t10874, t10878, t10881, t10885, t10888, t10891, t10893, t10896, t10900, t10902, t2721, t799, t825);
        let t10910 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1587(t10725, t10791, t10848, t10908);
    (t10887, t10888, t10890, t10891, t10893, t10896, t10899, t10900, t10902, t10905, t10906, t10910)
}
