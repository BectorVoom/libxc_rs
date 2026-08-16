//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta21 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk167;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk168;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk169;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk170;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk171;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk172;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta21(t30: f64, t33: f64, t265: f64, t395: f64, t45: f64, t57: f64, dens_threshold: f64, rho0: f64, rho1: f64, zeta_threshold: f64, t268: f64, t269: f64, t281: f64, t282: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t398, t403, t404) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk167(t30, t33, t265, t395, t45, t57, dens_threshold, rho0, rho1, zeta_threshold);
        let t406 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk168(t268, t269, t404);
        let t408 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk169(t406);
        let t409 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk170(t406);
        let (t412, t414) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk171(t406, t404);
        let (t416, t418, t421, t422) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk172(t281, t282, t414, t406, t409, t412);
    (t398, t403, t404, t406, t408, t409, t412, t414, t416, t418, t421, t422)
}
