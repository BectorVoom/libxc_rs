//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta644 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2093;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta644<F: Float>(t26865: F, t370: F, t17727: F, t17423: F, t29097: F, t17789: F, t29100: F, t17416: F, t7624: F, t17608: F, t7617: F, t17217: F, t26880: F) -> (F, F, F, F, F, F, F) {
        let (t104646, t104647, t104651, t104653, t104658, t104677, t104680) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2093::<F>(t26865, t370, t17727, t17423, t29097, t17789, t29100, t17416, t7624, t17608, t7617, t17217, t26880);
    (t104646, t104647, t104651, t104653, t104658, t104677, t104680)
}
