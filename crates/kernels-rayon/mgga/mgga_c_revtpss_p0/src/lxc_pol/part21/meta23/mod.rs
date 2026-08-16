//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta23 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk178;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk179;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk180;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk181;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk182;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk183;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk184;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta23(t406: f64, t456: f64, t344: f64, t56: f64, t404: f64, t221: f64, t65: f64, t225: f64, t355: f64, t424: f64, t452: f64, t454: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t458, t459) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk178(t406);
        let t460 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk179(t456, t459);
        let t461 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk180(t344, t56);
        let t462 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk181(t404);
        let (t464, t467, t471) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk182(t221, t462, t65, t225, t460, t355, t424, t452, t454);
        let (t472, t473) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk183(t471);
        let t474 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk184(sigma2);
    (t458, t459, t460, t461, t462, t464, t467, t471, t472, t473, t474)
}
