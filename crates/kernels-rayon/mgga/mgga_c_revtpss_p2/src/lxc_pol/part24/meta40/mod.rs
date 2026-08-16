//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta40 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk287;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk288;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk289;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk290;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk291;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta40(t686: f64, t874: f64, t875: f64, t251: f64, t822: f64, t261: f64, t159: f64, t675: f64, t268: f64, t271: f64, t373: f64, t631: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t878, t879) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk287(t686, t874, t875, t251, t822);
        let t892 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk288(t261);
        let (t900, t902) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk289(t159, t675, t268, t271);
        let (t903, t904) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk290(t902, t159, t373);
        let t905 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk291(t631);
    (t878, t879, t892, t900, t902, t903, t904, t905)
}
