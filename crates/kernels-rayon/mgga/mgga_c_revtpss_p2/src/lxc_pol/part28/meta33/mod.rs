//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta33 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk221;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk222;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk223;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk224;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk225;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk226;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk227;
use chunk7::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk228;
use chunk8::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk229;
use chunk9::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk230;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta33(t38: f64, t627: f64, t45: f64, t78: f64, t57: f64, t81: f64, t606: f64, t77: f64, t608: f64, t71: f64, t85: f64, t5: f64, t599: f64, t603: f64, t91: f64, t117: f64, t116: f64, t94: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t628, t631) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk221(t38, t627, t45);
        let t633 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk222(t631, t78);
        let t635 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk223(t57);
        let t637 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk224(t635, t81);
        let t640 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk225(t606, t633, t637);
        let t641 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk226(t640, t77);
        let t644 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk227(t608, t628, t641, t71, t85);
        let t648 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk228(t5, t599, t603, t644, t91);
        let t649 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk229(t117, t648);
        let t651 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk230(t116, t94);
    (t628, t631, t633, t635, t637, t640, t641, t644, t648, t649, t651)
}
