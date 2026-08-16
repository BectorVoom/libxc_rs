//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta650 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2101;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta650<F: Float>(t17445: F, t7607: F, t3655: F, t8177: F, t1256: F, t29074: F, t29069: F, t29089: F, t3685: F, t26948: F, t97065: F, t3555: F, t8190: F) -> (F, F, F, F, F, F, F) {
        let (t104994, t104999, t105002, t105007, t105014, t105046, t105134) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2101::<F>(t17445, t7607, t3655, t8177, t1256, t29074, t29069, t29089, t3685, t26948, t97065, t3555, t8190);
    (t104994, t104999, t105002, t105007, t105014, t105046, t105134)
}
