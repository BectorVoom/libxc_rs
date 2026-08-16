//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta23 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk174;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk175;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk176;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk177;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk178;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk179;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk180;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk181;
use chunk8::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk182;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta23(t406: f64, t456: f64, t344: f64, t56: f64, t404: f64, t221: f64, t65: f64, t225: f64, t355: f64, t424: f64, t452: f64, t454: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t458, t459) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk174(t406);
        let t460 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk175(t456, t459);
        let t461 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk176(t344, t56);
        let t462 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk177(t404);
        let (t464, t467) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk178(t221, t462, t65, t225, t460);
        let t471 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk179(t225, t355, t424, t452, t454);
        let (t472, t473) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk180(t471);
        let t474 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk181(sigma2);
        let t475 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk182(t473, t474);
    (t458, t459, t460, t461, t462, t464, t467, t471, t472, t473, t474, t475)
}
