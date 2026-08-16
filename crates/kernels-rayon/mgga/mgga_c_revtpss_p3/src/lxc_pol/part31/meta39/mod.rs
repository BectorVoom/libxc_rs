//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta39 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk254;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk255;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk256;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk257;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk258;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta39(t158: f64, t750: f64, t162: f64, t716: f64, t187: f64, t192: f64, t72: f64, t186: f64, t675: f64, t685: f64, t177: f64, t738: f64, t744: f64, t745: f64, t206: f64, t262: f64, t78: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t751, t752, t754, t755, t757) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk254(t158, t750, t162, t716, t187, t192, t72, t186, t675, t685);
        let (t759, t760) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk255(t755, t757, t177, t192);
        let t762 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk256(t738, t744, t745);
        let (t764, t765) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk257(t760, t762, t206, t262);
        let t766 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk258(t78);
    (t751, t752, t754, t755, t757, t759, t760, t762, t764, t765, t766)
}
