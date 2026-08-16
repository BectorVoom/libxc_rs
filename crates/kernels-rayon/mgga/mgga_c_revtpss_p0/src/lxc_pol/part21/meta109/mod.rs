//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta109 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk714;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk715;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk716;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk717;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk718;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk719;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta109(t225: f64, t2633: f64, t73: f64, t853: f64, t2394: f64, t2430: f64, t832: f64, t227: f64, t229: f64, t830: f64, t833: f64, t231: f64, t827: f64, t828: f64, t820: f64, t843: f64, t849: f64, t857: f64, t855: f64, t212: f64, t27: f64, t816: f64, t240: f64, t823: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2634, t2639, t2642, t2645) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk714(t225, t2633, t73, t853, t2394, t2430, t832, t227, t229, t830, t833);
        let t2646 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk715(t231, t2645);
        let (t2648, t2652) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk716(t2646, t827, t828, t820, t843, t849);
        let (t2653, t2656, t2659) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk717(t2652, t857, t2430, t828, t855, t212, t27);
        let t2661 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk718(t225, t2659, t816);
        let t2662 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk719(t240, t823);
    (t2634, t2639, t2642, t2645, t2646, t2648, t2652, t2653, t2656, t2659, t2661, t2662)
}
