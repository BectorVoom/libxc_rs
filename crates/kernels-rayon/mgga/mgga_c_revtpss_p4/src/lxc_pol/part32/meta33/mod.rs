//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta33 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk217;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk218;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk219;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk220;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk221;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk222;
use chunk6::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk223;
use chunk7::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk224;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta33(t635: f64, t81: f64, t606: f64, t633: f64, t77: f64, t608: f64, t628: f64, t71: f64, t85: f64, t5: f64, t599: f64, t603: f64, t91: f64, t117: f64, t116: f64, t94: f64, t112: f64, t625: f64, t111: f64, t43: f64, t605: f64, tau0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t637 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk217(t635, t81);
        let t640 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk218(t606, t633, t637);
        let (t641, t644) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk219(t640, t77, t608, t628, t71, t85);
        let t648 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk220(t5, t599, t603, t644, t91);
        let t649 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk221(t117, t648);
        let t651 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk222(t116, t94);
        let (t653, t654, t655) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk223(t112, t625, t111);
        let (t656, t658) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk224(t43, t605, tau0);
    (t637, t640, t641, t644, t648, t649, t651, t653, t654, t655, t656, t658)
}
