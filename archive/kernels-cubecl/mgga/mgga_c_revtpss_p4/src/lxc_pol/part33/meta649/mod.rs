//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta649 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2099;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2100;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta649<F: Float>(t17361: F, t7618: F, t17289: F, t2138: F, t3666: F, t8184: F, t17451: F, t26867: F, t1285: F, t97173: F, t104646: F, t17735: F, t17617: F, t26870: F, t3682: F, t8172: F, t29020: F, t3704: F, t29086: F, t3678: F, t3655: F, t8185: F, t17628: F, t7607: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t104905, t104916, t104924, t104933, t104943, t104946) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2099::<F>(t17361, t7618, t17289, t2138, t3666, t8184, t17451, t26867, t1285, t97173, t104646, t17735);
        let (t104953, t104963, t104968, t104972, t104988, t104990) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2100::<F>(t17617, t26870, t3682, t8172, t29020, t3704, t29086, t3678, t3655, t8185, t17628, t7607);
    (t104905, t104916, t104924, t104933, t104943, t104946, t104953, t104963, t104968, t104972, t104988, t104990)
}
