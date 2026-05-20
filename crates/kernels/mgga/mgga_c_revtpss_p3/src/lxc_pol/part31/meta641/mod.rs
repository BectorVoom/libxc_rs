//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta641 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2098;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta641<F: Float>(t98285: F, t98964: F, t98976: F, t98979: F, t99009: F, t99013: F, t99035: F, t99044: F, t99050: F, t99091: F, t99113: F, t30160: F, t575: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t102569, t103264, t103269, t103270, t103285, t103287, t103297, t103302, t103305, t103329, t103347, t105814) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2098::<F>(t98285, t98964, t98976, t98979, t99009, t99013, t99035, t99044, t99050, t99091, t99113, t30160, t575);
    (t102569, t103264, t103269, t103270, t103285, t103287, t103297, t103302, t103305, t103329, t103347, t105814)
}
