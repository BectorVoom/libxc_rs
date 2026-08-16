//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta31 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk196;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk197;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk198;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk199;
use chunk4::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk200;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta31<F: Float>(t25: F, t596: F, t578: F, t582: F, t586: F, t590: F, t594: F, t88: F, t90: F, t29: F, t17: F, t2: F, t4: F, t30: F, t33: F, zeta_threshold: F, t36: F, t70: F, t39: F, t41: F, rho0: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t598, t599, t602) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk196::<F>(t25, t596, t578, t582, t586, t590, t594, t88, t90);
        let t603 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk197::<F>(t29, t602);
        let (t604, t605) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk198::<F>(t17, t2, t4);
        let t606 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk199::<F>(t30, t33, t605, zeta_threshold);
        let (t607, t608, t614) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk200::<F>(t36, t606, t70, t39, t41, rho0, sigma0);
    (t598, t599, t602, t603, t604, t605, t606, t607, t608, t614)
}
