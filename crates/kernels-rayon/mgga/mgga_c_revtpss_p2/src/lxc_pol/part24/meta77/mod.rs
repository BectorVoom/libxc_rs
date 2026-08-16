//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta77 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk466;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk467;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk468;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk469;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk470;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta77(t200: f64, t202: f64, t205: f64, t262: f64, t198: f64, t206: f64, t261: f64, t125: f64, t215: f64, t123: f64, t781: f64, t124: f64, t68: f64, t138: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2375, t2382, t2393, t2403) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk466(t200, t202, t205, t262, t198, t206);
        let (t2410, t2411) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk467(t261);
        let t2434 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk468(t125, t215);
        let t2435 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk469(t123, t2434);
        let (t2437, t2438, t2439) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk470(t2435, t781, t124, t68, t138);
    (t2375, t2382, t2393, t2403, t2410, t2411, t2434, t2435, t2437, t2438, t2439)
}
