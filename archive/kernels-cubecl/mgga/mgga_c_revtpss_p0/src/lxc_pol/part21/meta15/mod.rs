//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta15 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk123;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk124;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk125;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk126;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk127;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk128;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk129;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta15<F: Float>(t123: F, t125: F, t126: F, t159: F, t45: F, t124: F, t138: F, t139: F, t240: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t268 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk123::<F>(t123, t125);
        let (t269, t270, t271) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk124::<F>(t126, t159, t45);
        let t273 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk125::<F>(t268, t269, t271);
        let t275 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk126::<F>(t273);
        let t276 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk127::<F>(t273);
        let (t279, t281) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk128::<F>(t273, t124, t138);
        let (t282, t283) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk129::<F>(t139, t240, t271);
    (t268, t269, t270, t271, t273, t275, t276, t279, t281, t282, t283)
}
