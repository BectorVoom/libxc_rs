//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta92 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk634;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk635;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk636;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk637;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk638;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk639;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk640;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk641;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta92(t198: f64, t206: f64, t890: f64, t892: f64, t261: f64, t125: f64, t215: f64, t123: f64, t781: f64, t124: f64, t68: f64, t138: f64, t251: f64, t785: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t2403 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk634(t198, t206);
        let t2404 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk635(t890, t892);
        let (t2410, t2411) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk636(t261);
        let t2434 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk637(t125, t215);
        let t2435 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk638(t123, t2434);
        let (t2437, t2438) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk639(t2435, t781, t124, t68);
        let t2439 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk640(t138, t2438);
        let t2440 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk641(t251, t785);
    (t2403, t2404, t2410, t2411, t2434, t2435, t2437, t2438, t2439, t2440)
}
