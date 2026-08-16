//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta33 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk243;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk244;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk245;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk246;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk247;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk248;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta33(t36: f64, t37: f64, t157: f64, t169: f64, t164: f64, t687: f64, t689: f64, t693: f64, t698: f64, t172: f64, t182: f64, t177: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t705, t706) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk243(t36, t37, t157);
        let (t722, t723, t724, t729) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk244(t169, t164, t687, t689, t693, t698);
        let t730 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk245(t172);
        let t731 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk246(t729, t730);
        let (t737, t738) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk247(t182);
        let (t739, t744) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk248(t177, t738, t687, t689, t693, t698);
    (t705, t706, t722, t723, t724, t729, t730, t731, t737, t738, t739, t744)
}
