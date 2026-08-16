//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta226 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1440;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1441;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1442;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta226(t198: f64, t530: f64, t1868: f64, t566: f64, t532: f64, t1907: f64, t4147: f64) -> (f64, f64, f64, f64) {
        let t5536 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1440(t198, t530);
        let (t5537, t5541) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1441(t1868, t566, t198, t532);
        let t5542 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1442(t1907, t4147);
    (t5536, t5537, t5541, t5542)
}
