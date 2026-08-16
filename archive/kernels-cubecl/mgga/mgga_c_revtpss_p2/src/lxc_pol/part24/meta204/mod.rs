//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta204 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk938;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta204<F: Float>(t760: F, t9419: F, t9387: F, t9372: F, t9425: F, t2475: F, t73: F, t2710: F, t2793: F, t9285: F, t874: F, t875: F, t9288: F) -> (F, F, F, F, F, F, F) {
        let (t10592, t10596, t10604, t10611, t10626, t10645, t10651) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk938::<F>(t760, t9419, t9387, t9372, t9425, t2475, t73, t2710, t2793, t9285, t874, t875, t9288);
    (t10592, t10596, t10604, t10611, t10626, t10645, t10651)
}
