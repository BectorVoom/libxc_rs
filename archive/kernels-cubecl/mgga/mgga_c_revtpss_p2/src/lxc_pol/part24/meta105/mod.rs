//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta105 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk599;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk600;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk601;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk602;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk603;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk604;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk605;
use chunk7::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk606;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta105<F: Float>(t3147: F, t479: F, t3597: F, t3594: F, t471: F, t3153: F, t1244: F, t1121: F, t414: F, t66: F, t474: F, t3089: F, t1285: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3598, t3599, t3600, t3603) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk599::<F>(t3147, t479, t3597, t3594, t471);
        let t3604 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk600::<F>(t3153, t3603);
        let (t3609, t3610, t3611) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk601::<F>(t1244, t3598, t3594, t3153, t471);
        let t3617 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk602::<F>(t1121, t414);
        let t3618 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk603::<F>(t3617, t66);
        let t3623 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk604::<F>(t474, t479);
        let t3624 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk605::<F>(t3089, t3623);
        let t3625 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk606::<F>(t1285, t3624);
    (t3599, t3600, t3603, t3604, t3609, t3610, t3611, t3617, t3618, t3623, t3624, t3625)
}
