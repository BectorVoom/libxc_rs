//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta32 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk236;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk237;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk238;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk239;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk240;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk241;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk242;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta32(t684: f64, t686: f64, t123: f64, t676: f64, t128: f64, t72: f64, t3: f64, t66: f64, t124: f64, t138: f64, t146: f64, t682: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t687, t689) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk236(t684, t686, t123, t676);
        let (t692, t693, t696, t697) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk237(t128, t72, t686, t3, t66, t124);
        let t698 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk238(t138, t697);
        let t700 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk239(t687, t689, t693, t698);
        let t701 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk240(t146);
        let t702 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk241(t700, t701);
        let t704 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk242(t682, t702);
    (t687, t689, t692, t693, t696, t697, t698, t700, t701, t702, t704)
}
