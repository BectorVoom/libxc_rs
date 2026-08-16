//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta21 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk147;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk148;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk149;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk150;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk151;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk152;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta21(t342: f64, t386: f64, t198: f64, t293: f64, t328: f64, t330: f64, t336: f64, t265: f64, t30: f64, t33: f64, t45: f64, t57: f64, dens_threshold: f64, rho0: f64, rho1: f64, zeta_threshold: f64, t268: f64, t269: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t389, t395, t393) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk147(t342, t386, t198, t293, t328, t330, t336, t265);
        let (t398, t403, t404) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk148(t30, t33, t265, t395, t45, t57, dens_threshold, rho0, rho1, zeta_threshold);
        let t406 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk149(t268, t269, t404);
        let t408 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk150(t406);
        let t409 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk151(t406);
        let (t412, t414) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk152(t406, t404);
    (t389, t395, t393, t398, t403, t404, t406, t408, t409, t412, t414)
}
