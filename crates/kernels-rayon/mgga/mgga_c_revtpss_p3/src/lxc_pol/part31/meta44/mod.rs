//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta44 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk288;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk289;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk290;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk291;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk292;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk293;
use chunk6::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk294;
use chunk7::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk295;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta44(t243: f64, t844: f64, t247: f64, t237: f64, t233: f64, t235: f64, t239: f64, t820: f64, t205: f64, t242: f64, t240: f64, t72: f64, t775: f64, t828: f64, t797: f64, t799: f64, t802: f64, t812: f64, t819: f64, t825: f64, t839: f64, t225: f64, t257: f64, t213: f64, t251: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t846, t848, t849) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk288(t243, t844, t247, t237, t233, t235);
        let t851 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk289(t239, t820, t849);
        let t853 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk290(t205, t242);
        let t854 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk291(t240, t853);
        let t855 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk292(t72, t854);
        let t857 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk293(t775, t828, t855);
        let t860 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk294(t797, t799, t802, t812, t819, t825, t839, t848, t851, t857);
        let (t861, t862, t865) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk295(t225, t860, t257, t213, t251);
    (t846, t848, t849, t851, t853, t854, t855, t857, t860, t861, t862, t865)
}
