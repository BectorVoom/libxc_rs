//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta488 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1782;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1783;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1784;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1785;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1786;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1787;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta488(t1971: f64, t3104: f64, t351: f64, t25516: f64, t3114: f64, t3057: f64, t7143: f64, t1035: f64, t8515: f64, t1983: f64, t378: f64, t7150: f64, t8521: f64, t995: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25576, t25577, t25580) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1782(t1971, t3104, t351, t25516, t3114);
        let t25591 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1783(t3057, t7143);
        let t25604 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1784(t1035, t8515);
        let t25605 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1785(t1983, t25604);
        let (t25610, t25611) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1786(t378, t7150, t8521);
        let t25629 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1787(t8521, t995);
    (t25576, t25577, t25580, t25591, t25604, t25605, t25610, t25611, t25629)
}
