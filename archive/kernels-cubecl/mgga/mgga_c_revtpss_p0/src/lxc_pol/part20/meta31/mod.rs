//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta31 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk233;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk234;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk235;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk236;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk237;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk238;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta31<F: Float>(t57: F, t81: F, t606: F, t633: F, t77: F, t608: F, t628: F, t71: F, t85: F, t5: F, t599: F, t603: F, t91: F, t117: F, t116: F, t94: F) -> (F, F, F, F, F, F, F, F) {
        let t635 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk233::<F>(t57);
        let t637 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk234::<F>(t635, t81);
        let (t640, t641) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk235::<F>(t606, t633, t637, t77);
        let t644 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk236::<F>(t608, t628, t641, t71, t85);
        let (t648, t649) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk237::<F>(t5, t599, t603, t644, t91, t117);
        let t651 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk238::<F>(t116, t94);
    (t635, t637, t640, t641, t644, t648, t649, t651)
}
