//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta37 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk246;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk247;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta37(t45: f64, t57: f64, t190: f64, t606: f64, t706: f64, t78: f64, t81: f64, t150: f64, t169: f64, t164: f64, t687: f64, t689: f64, t693: f64, t698: f64, zeta_threshold: f64, t172: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t707, t709, t716, t717, t718, t722, t723, t724, t729) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk246(t45, t57, t190, t606, t706, t78, t81, t150, t169, t164, t687, t689, t693, t698, zeta_threshold);
        let t730 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk247(t172);
    (t707, t709, t716, t717, t718, t722, t723, t724, t729, t730)
}
