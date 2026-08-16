//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta826 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2945;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta826<F: Float>(t5571: F, t9387: F, t13613: F, t2619: F, t9323: F, t13581: F, t72: F, t757: F, t5635: F, t9586: F, t9425: F, t9318: F) -> (F, F, F, F, F, F, F) {
        let (t48262, t48267, t48269, t48277, t48280, t48282, t48285) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2945::<F>(t5571, t9387, t13613, t2619, t9323, t13581, t72, t757, t5635, t9586, t9425, t9318);
    (t48262, t48267, t48269, t48277, t48280, t48282, t48285)
}
