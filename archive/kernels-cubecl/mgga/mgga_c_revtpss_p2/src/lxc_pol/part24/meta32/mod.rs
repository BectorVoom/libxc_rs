//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta32 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk236;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk237;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk238;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk239;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk240;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk241;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk242;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta32<F: Float>(t684: F, t686: F, t123: F, t676: F, t128: F, t72: F, t3: F, t66: F, t124: F, t138: F, t146: F, t682: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t687, t689) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk236::<F>(t684, t686, t123, t676);
        let (t692, t693, t696, t697) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk237::<F>(t128, t72, t686, t3, t66, t124);
        let t698 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk238::<F>(t138, t697);
        let t700 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk239::<F>(t687, t689, t693, t698);
        let t701 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk240::<F>(t146);
        let t702 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk241::<F>(t700, t701);
        let t704 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk242::<F>(t682, t702);
    (t687, t689, t692, t693, t696, t697, t698, t700, t701, t702, t704)
}
