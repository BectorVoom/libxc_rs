//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta841 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3153;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta841<F: Float>(t56228: F, t1132: F, t58106: F, t1134: F, t3399: F, t16851: F, t16854: F, t2439: F, t5101: F, t16870: F, t698: F, t56221: F, t56226: F, t56230: F, t56234: F, t56236: F) -> (F, F, F, F, F, F) {
        let (t58138, t58141, t58143, t58145, t58147, t58149) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3153::<F>(t56228, t1132, t58106, t1134, t3399, t16851, t16854, t2439, t5101, t16870, t698, t56221, t56226, t56230, t56234, t56236);
    (t58138, t58141, t58143, t58145, t58147, t58149)
}
