//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta75 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk453;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk454;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk455;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk456;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta75(t117: f64, t670: f64, t1459: f64, t572: f64, t573: f64, t578: f64, t582: f64, t586: f64, t590: f64, t594: f64, t598: f64, t4: f64, t604: f64, t30: f64, t33: f64, zeta_threshold: f64, t36: f64, t70: f64, t48: f64, t51: f64, t53: f64, rho1: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1461 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk453(t117, t670);
        let (t1464, t1466, t1468) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk454(t1459, t1461, t572, t573, t578, t582, t586, t590, t594, t598, t4, t604);
        let t1469 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk455(t30, t33, t1468, zeta_threshold);
        let (t1470, t1471, t1474, t1480) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk456(t1469, t36, t70, t48, t51, t53, rho1, sigma2);
    (t1461, t1464, t1466, t1468, t1469, t1470, t1471, t1474, t1480)
}
