//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta98 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk625;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk626;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk627;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk628;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta98(t99: f64, t107: f64, t200: f64, t202: f64, t205: f64, t262: f64, t705: f64, t716: f64, t198: f64, t206: f64, t890: f64, t892: f64, t261: f64, t125: f64, t215: f64, t123: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2349, t2357, t2375, t2382, t2393, t2398, t2403) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk625(t99, t107, t200, t202, t205, t262, t705, t716, t198, t206);
        let (t2404, t2410, t2411) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk626(t890, t892, t261);
        let t2434 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk627(t125, t215);
        let t2435 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk628(t123, t2434);
    (t2349, t2357, t2375, t2382, t2393, t2398, t2403, t2404, t2410, t2411, t2434, t2435)
}
