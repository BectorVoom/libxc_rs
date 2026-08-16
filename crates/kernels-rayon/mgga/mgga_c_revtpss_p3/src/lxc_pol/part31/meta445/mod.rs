//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta445 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1590;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1591;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1592;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta445(t18280: f64, t3531: f64, t6556: f64, t6552: f64, t3362: f64, t5825: f64, t606: f64, t3417: f64, t141: f64, t1121: f64, t18281: f64, t1145: f64, t6461: f64, t698: f64, t6464: f64, t6467: f64, t6422: f64, t689: f64, t6426: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20256, t20261, t20263, t20266, t20268, t20272, t20273) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1590(t18280, t3531, t6556, t6552, t3362, t5825, t606, t3417, t141, t1121, t18281, t1145);
        let (t20274, t20276, t20278, t20280, t20283) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1591(t141, t20273, t6461, t698, t6464, t6467, t6422, t689);
        let t20285 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1592(t6426, t689);
    (t20256, t20261, t20263, t20266, t20268, t20272, t20274, t20276, t20278, t20280, t20283, t20285)
}
