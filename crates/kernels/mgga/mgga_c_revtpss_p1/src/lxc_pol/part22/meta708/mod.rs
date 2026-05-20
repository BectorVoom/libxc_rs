//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta708 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2731;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta708<F: Float>(t1014: F, t65: F, t3252: F, t1513: F, t665: F, t1224: F, t3698: F, t10208: F, t69: F, t1504: F, t658: F, t1509: F, t661: F) -> (F, F, F, F, F, F, F, F) {
        let (t27527, t27531, t28036, t29048, t29054, t31035, t31283, t31443) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2731::<F>(t1014, t65, t3252, t1513, t665, t1224, t3698, t10208, t69, t1504, t658, t1509, t661);
    (t27527, t27531, t28036, t29048, t29054, t31035, t31283, t31443)
}
