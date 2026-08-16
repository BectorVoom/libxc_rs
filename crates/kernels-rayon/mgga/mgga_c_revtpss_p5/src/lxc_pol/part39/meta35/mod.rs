//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta35 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk220;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk221;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk222;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk223;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta35(t143: f64, t130: f64, t131: f64, t72: f64, t122: f64, t125: f64, t675: f64, t123: f64, t676: f64, t128: f64, t3: f64, t66: f64, t124: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t680, t681, t682, t684, t685) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk220(t143, t130, t131, t72, t122, t125);
        let t686 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk221(t675, t685);
        let (t687, t689) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk222(t684, t686, t123, t676);
        let (t692, t693, t696, t697) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk223(t128, t72, t686, t3, t66, t124);
    (t680, t681, t682, t684, t685, t686, t687, t689, t692, t693, t696, t697)
}
