//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta178 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk879;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk880;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk881;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk882;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta178<F: Float>(t684: F, t9288: F, t125: F, t793: F, t123: F, t128: F, t121: F, t22: F, t2508: F, t9285: F, t692: F, t124: F, t624: F, t138: F, t9283: F, t9286: F, t701: F, t682: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t9289, t9291) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk879::<F>(t684, t9288, t125, t793);
        let t9292 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk880::<F>(t123, t9291);
        let (t9295, t9296, t9298, t9300, t9302, t9303) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk881::<F>(t128, t121, t22, t2508, t9285, t692, t9288, t124, t624, t138);
        let (t9305, t9306, t9308) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk882::<F>(t9283, t9286, t9289, t9292, t9296, t9298, t9300, t9303, t701, t682);
    (t9289, t9291, t9292, t9295, t9296, t9298, t9300, t9302, t9303, t9305, t9306, t9308)
}
