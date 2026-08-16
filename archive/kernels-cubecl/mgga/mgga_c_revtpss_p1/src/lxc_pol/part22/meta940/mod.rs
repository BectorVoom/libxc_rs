//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta940 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3175;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta940<F: Float>(t16900: F, t698: F, t2439: F, t5095: F, t16903: F, t16907: F, t16886: F, t16889: F, t5098: F, t1179: F, t16831: F, t1744: F, t3477: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t58162, t58165, t58186, t58207, t58209, t58211, t58225, t58234, t58237) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3175::<F>(t16900, t698, t2439, t5095, t16903, t16907, t16886, t16889, t5098, t1179, t16831, t1744, t3477);
    (t58162, t58165, t58186, t58207, t58209, t58211, t58225, t58234, t58237)
}
