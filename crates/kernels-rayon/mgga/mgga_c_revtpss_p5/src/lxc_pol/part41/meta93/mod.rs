//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta93 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk527;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk528;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk529;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta93(t2289: f64, t45: f64, t631: f64, t78: f64, t57: f64, t635: f64, t81: f64, t116: f64, t648: f64, t112: f64, t625: f64, t666: f64, t111: f64, t654: f64, t99: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2290, t2297, t2299, t2304, t2306, t2322) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk527(t2289, t45, t631, t78, t57, t635, t81, t116, t648);
        let (t2335, t2336, t2339) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk528(t112, t2289, t625, t666, t111, t654);
        let t2349 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk529(t99);
    (t2290, t2297, t2299, t2304, t2306, t2322, t2335, t2336, t2339, t2349)
}
