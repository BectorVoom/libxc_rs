//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta36 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk263;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk264;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk265;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk266;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk267;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta36(t150: f64, t716: f64, t190: f64, t169: f64, t164: f64, t687: f64, t689: f64, t693: f64, t698: f64, t172: f64, t182: f64, t177: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t717, t718, t722, t723, t724, t729) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk263(t150, t716, t190, t169, t164, t687, t689, t693, t698);
        let t730 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk264(t172);
        let t731 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk265(t729, t730);
        let (t737, t738) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk266(t182);
        let (t739, t744) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk267(t177, t738, t687, t689, t693, t698);
    (t717, t718, t722, t723, t724, t729, t730, t731, t737, t738, t739, t744)
}
