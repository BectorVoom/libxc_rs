//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1030 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3613;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1030<F: Float>(t20361: F, t3399: F, t20365: F, t16926: F, t5087: F, t1134: F, t20337: F, t3407: F, t20370: F, t20356: F, t58145: F, t58147: F, t68470: F, t68473: F, t68476: F, t68479: F, t68481: F, t68484: F) -> (F, F, F, F, F, F, F) {
        let (t68486, t68488, t68490, t68493, t68495, t68497, t68501) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3613::<F>(t20361, t3399, t20365, t16926, t5087, t1134, t20337, t3407, t20370, t20356, t58145, t58147, t68470, t68473, t68476, t68479, t68481, t68484);
    (t68486, t68488, t68490, t68493, t68495, t68497, t68501)
}
