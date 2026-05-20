//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta787 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2835;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta787<F: Float>(t141: F, t2908: F, t51905: F, t15183: F, t698: F, t15172: F, t2439: F, t4625: F, t4622: F, t15186: F, t51890: F, t51892: F, t51894: F, t51896: F, t51899: F, t51902: F) -> (F, F, F, F, F, F, F) {
        let (t51907, t51909, t51911, t51913, t51915, t51917, t51919) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2835::<F>(t141, t2908, t51905, t15183, t698, t15172, t2439, t4625, t4622, t15186, t51890, t51892, t51894, t51896, t51899, t51902);
    (t51907, t51909, t51911, t51913, t51915, t51917, t51919)
}
