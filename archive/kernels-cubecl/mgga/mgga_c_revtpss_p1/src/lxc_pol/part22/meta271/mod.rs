//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta271 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1657;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1658;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1659;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta271<F: Float>(t98: F, t106: F, t143: F, t2580: F, t130: F, t2566: F, t700: F, t2584: F, t121: F, t131: F, t141: F, t22: F, t2456: F, t624: F, t2501: F, t685: F, t793: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t9163, t9232, t9273, t9274, t9275, t9276, t9278) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1657::<F>(t98, t106, t143, t2580, t130, t2566, t700, t2584);
        let (t9282, t9283, t9285) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1658::<F>(t121, t131, t141, t22, t2456, t624);
        let (t9286, t9288) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1659::<F>(t2501, t9285, t685, t793);
    (t9163, t9232, t9273, t9274, t9275, t9276, t9278, t9282, t9283, t9285, t9286, t9288)
}
