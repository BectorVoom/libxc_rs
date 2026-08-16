//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta80 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk489;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk490;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk491;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta80(t123: f64, t2434: f64, t781: f64, t124: f64, t68: f64, t138: f64) -> (f64, f64, f64, f64) {
        let t2435 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk489(t123, t2434);
        let (t2437, t2438) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk490(t2435, t781, t124, t68);
        let t2439 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk491(t138, t2438);
    (t2435, t2437, t2438, t2439)
}
