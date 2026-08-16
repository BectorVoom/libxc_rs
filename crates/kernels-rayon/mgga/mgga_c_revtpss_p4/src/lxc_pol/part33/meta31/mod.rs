//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta31 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk213;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk214;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk215;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk216;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk217;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk218;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk219;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta31(t29: f64, t602: f64, t17: f64, t2: f64, t4: f64, t30: f64, t33: f64, zeta_threshold: f64, t36: f64, t70: f64, t39: f64, t41: f64, rho0: f64, sigma0: f64, t48: f64, t60: f64, t579: f64, t66: f64, t64: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t603 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk213(t29, t602);
        let (t604, t605) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk214(t17, t2, t4);
        let t606 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk215(t30, t33, t605, zeta_threshold);
        let t607 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk216(t36, t606);
        let (t608, t614) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk217(t607, t70, t39, t41, rho0, sigma0);
        let (t617, t620, t624) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk218(t48, t606, t60, t579, t66);
        let t625 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk219(t624, t64);
    (t603, t604, t605, t606, t607, t608, t614, t617, t620, t624, t625)
}
