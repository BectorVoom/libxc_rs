//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta75 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk453;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk454;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk455;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk456;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta75<F: Float>(t117: F, t670: F, t1459: F, t572: F, t573: F, t578: F, t582: F, t586: F, t590: F, t594: F, t598: F, t4: F, t604: F, t30: F, t33: F, zeta_threshold: F, t36: F, t70: F, t48: F, t51: F, t53: F, rho1: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F) {
        let t1461 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk453::<F>(t117, t670);
        let (t1464, t1466, t1468) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk454::<F>(t1459, t1461, t572, t573, t578, t582, t586, t590, t594, t598, t4, t604);
        let t1469 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk455::<F>(t30, t33, t1468, zeta_threshold);
        let (t1470, t1471, t1474, t1480) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk456::<F>(t1469, t36, t70, t48, t51, t53, rho1, sigma2);
    (t1461, t1464, t1466, t1468, t1469, t1470, t1471, t1474, t1480)
}
