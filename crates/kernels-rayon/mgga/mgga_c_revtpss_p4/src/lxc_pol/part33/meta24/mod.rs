//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta24 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk167;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk168;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk169;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk170;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk171;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk172;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk173;
use chunk7::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk174;
use chunk8::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk175;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta24(t225: f64, t460: f64, t355: f64, t424: f64, t452: f64, t454: f64, sigma2: f64, t51: f64, t52: f64, rho1: f64, t414: f64, t371: f64, t372: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t467 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk167(t225, t460);
        let t471 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk168(t225, t355, t424, t452, t454);
        let (t472, t473) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk169(t471);
        let t474 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk170(sigma2);
        let t475 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk171(t473, t474);
        let (t476, t479) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk172(t51, t52, rho1);
        let t480 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk173(t475, t479);
        let (t481, t482) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk174(t467, t480, t414);
        let t484 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk175(t371, t372, t482);
    (t467, t471, t472, t473, t474, t475, t476, t479, t480, t481, t482, t484)
}
