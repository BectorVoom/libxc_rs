//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta95 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk532;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk533;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk534;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk535;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta95(t2246: f64, t29: f64, t644: f64, t606: f64, t70: f64, t2: f64, t580: f64, t17: f64, t30: f64, t33: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2247, t2248, t2251) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk532(t2246, t29, t644, t606);
        let (t2252, t2255, t2256) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk533(t2251, t70, t2, t580, t17);
        let t2257 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk534(t2256);
        let t2258 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk535(t30, t33, t2257, zeta_threshold);
    (t2247, t2248, t2251, t2252, t2255, t2256, t2257, t2258)
}
