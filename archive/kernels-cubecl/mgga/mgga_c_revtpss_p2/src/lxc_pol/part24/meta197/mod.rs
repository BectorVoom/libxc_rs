//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta197 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk931;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta197<F: Float>(t10115: F, t557: F, t1429: F, t9292: F, t3964: F, t4096: F, t9285: F, t2453: F, t4100: F, t562: F, t64: F, t843: F) -> (F, F, F, F, F, F) {
        let (t10117, t10126, t10129, t10139, t10157, t10199) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk931::<F>(t10115, t557, t1429, t9292, t3964, t4096, t9285, t2453, t4100, t562, t64, t843);
    (t10117, t10126, t10129, t10139, t10157, t10199)
}
