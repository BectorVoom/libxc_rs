//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta8 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk65;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk66;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk67;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk68;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk69;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk70;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk71;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta8<F: Float>(t128: F, t72: F, t122: F, t66: F, t124: F, t131: F, t130: F, t37: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t134, t136) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk65::<F>(t128, t72);
        let (t137, t138) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk66::<F>(t122, t136);
        let (t139, t140) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk67::<F>(t66, t124);
        let t141 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk68::<F>(t138, t140);
        let (t143, t146, t147) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk69::<F>(t128, t131, t134, t141);
        let t149 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk70::<F>(t130, t147);
        let t150 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk71::<F>(t37);
    (t134, t136, t137, t138, t139, t140, t141, t143, t146, t147, t149, t150)
}
