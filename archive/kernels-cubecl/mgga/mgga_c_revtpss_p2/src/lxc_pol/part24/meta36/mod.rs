//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta36 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk260;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk261;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk262;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk263;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk264;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta36<F: Float>(t252: F, t786: F, t257: F, t72: F, t686: F, t579: F, t65: F, t64: F, t159: F, t222: F, t228: F, t216: F, t136: F, t220: F, t124: F, t212: F, t27: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t787, t788) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk260::<F>(t252, t786, t257, t72);
        let (t789, t791, t793) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk261::<F>(t686, t788, t787, t579, t65);
        let t794 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk262::<F>(t64, t793);
        let (t795, t797, t798, t799, t800) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk263::<F>(t159, t794, t222, t228, t216, t136, t220);
        let t807 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk264::<F>(t124, t212, t27, t800);
    (t787, t788, t789, t791, t793, t794, t795, t797, t798, t799, t800, t807)
}
