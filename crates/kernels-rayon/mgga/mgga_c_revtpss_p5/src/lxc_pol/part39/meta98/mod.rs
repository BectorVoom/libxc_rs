//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta98 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk542;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk543;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk544;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta98(t2327: f64, t94: f64, t1310: f64, t670: f64, t112: f64, t2289: f64, t625: f64, t666: f64, t111: f64, t654: f64, t665: f64, t613: f64, t99: f64, tau0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2328, t2331, t2335, t2336, t2339) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk542(t2327, t94, t1310, t670, t112, t2289, t625, t666, t111, t654);
        let t2340 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk543(t665);
        let (t2341, t2344, t2349) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk544(t2339, t2340, t613, t99, tau0);
    (t2328, t2331, t2335, t2336, t2339, t2340, t2341, t2344, t2349)
}
