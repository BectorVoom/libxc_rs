//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta405 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1869;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta405<F: Float>(t13043: F, t482: F, t3603: F, t471: F) -> (F, F) {
        let (t13044, t13045) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1869::<F>(t13043, t482, t3603, t471);
    (t13044, t13045)
}
