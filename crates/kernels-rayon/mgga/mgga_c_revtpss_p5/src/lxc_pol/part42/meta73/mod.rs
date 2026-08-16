//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta73 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk442;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk443;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk444;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk445;
use chunk4::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk446;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta73(t118: f64, t1310: f64, t1315: f64, t1453: f64, t508: f64, t511: f64, t569: f64, t649: f64, t651: f64, t671: f64, t3: f64, t571: f64, param_d: f64, t117: f64, t670: f64, t572: f64, t573: f64, t578: f64, t582: f64, t586: f64, t590: f64, t594: f64, t598: f64, t4: f64, t604: f64, t30: f64, t33: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1455, t1456, t1458) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk442(t118, t1310, t1315, t1453, t508, t511, t569, t649, t651, t671, t3, t571);
        let t1459 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk443(t1455, param_d);
        let t1461 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk444(t117, t670);
        let (t1464, t1466, t1468) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk445(t1459, t1461, t572, t573, t578, t582, t586, t590, t594, t598, t4, t604);
        let t1469 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk446(t30, t33, t1468, zeta_threshold);
    (t1455, t1456, t1458, t1459, t1461, t1464, t1466, t1468, t1469)
}
