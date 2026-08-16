//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta93 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk605;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk606;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk607;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk608;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk609;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk610;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta93(t30: f64, t1966: f64, t2129: f64, t45: f64, t343: f64, t55: f64, t136: f64, t473: f64, dens_threshold: f64, rho0: f64, sigma2: f64, zeta_threshold: f64, t479: f64, t467: f64, t464: f64, t484: f64, t225: f64, t494: f64, t456: f64, t1208: f64, t487: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2132, t2133, t2134, t2137) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk605(t30, t1966, t2129, t45, t343, t55, t136, t473, dens_threshold, rho0, sigma2, zeta_threshold);
        let t2138 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk606(t2137, t479);
        let (t2139, t2142) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk607(t2138, t467, t2134, t464, t484);
        let (t2143, t2144) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk608(t2142, t225, t494);
        let (t2147, t2148) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk609(t456, t1208);
        let t2149 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk610(t2148, t487);
    (t2132, t2133, t2134, t2137, t2138, t2139, t2142, t2143, t2144, t2147, t2148, t2149)
}
