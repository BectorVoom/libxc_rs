//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta31 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk196;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk197;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk198;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk199;
use chunk4::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk200;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta31(t25: f64, t596: f64, t578: f64, t582: f64, t586: f64, t590: f64, t594: f64, t88: f64, t90: f64, t29: f64, t17: f64, t2: f64, t4: f64, t30: f64, t33: f64, zeta_threshold: f64, t36: f64, t70: f64, t39: f64, t41: f64, rho0: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t598, t599, t602) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk196(t25, t596, t578, t582, t586, t590, t594, t88, t90);
        let t603 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk197(t29, t602);
        let (t604, t605) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk198(t17, t2, t4);
        let t606 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk199(t30, t33, t605, zeta_threshold);
        let (t607, t608, t614) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk200(t36, t606, t70, t39, t41, rho0, sigma0);
    (t598, t599, t602, t603, t604, t605, t606, t607, t608, t614)
}
