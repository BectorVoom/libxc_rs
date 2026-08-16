//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta34 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk247;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk248;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk249;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk250;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk251;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk252;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk253;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta34<F: Float>(t128: F, t72: F, t686: F, t3: F, t66: F, t124: F, t138: F, t687: F, t689: F, t146: F, t682: F, t36: F, t37: F, t157: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t692, t693, t696, t697) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk247::<F>(t128, t72, t686, t3, t66, t124);
        let t698 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk248::<F>(t138, t697);
        let t700 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk249::<F>(t687, t689, t693, t698);
        let t701 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk250::<F>(t146);
        let t702 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk251::<F>(t700, t701);
        let t704 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk252::<F>(t682, t702);
        let (t705, t706) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk253::<F>(t36, t37, t157);
    (t692, t693, t696, t697, t698, t700, t701, t702, t704, t705, t706)
}
