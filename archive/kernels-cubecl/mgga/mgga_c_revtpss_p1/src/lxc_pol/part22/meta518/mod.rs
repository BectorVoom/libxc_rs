//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta518 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2285;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta518<F: Float>(t141: F, t16886: F, t1145: F, t16733: F, t5098: F, t698: F, t16725: F, t3417: F, t16729: F, t16720: F, t16738: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t16887, t16889, t16890, t16892, t16893, t16894, t16895, t16897, t16898, t16900, t16901, t16903) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2285::<F>(t141, t16886, t1145, t16733, t5098, t698, t16725, t3417, t16729, t16720, t16738);
    (t16887, t16889, t16890, t16892, t16893, t16894, t16895, t16897, t16898, t16900, t16901, t16903)
}
