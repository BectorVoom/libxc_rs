//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta552 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2109;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta552<F: Float>(t14239: F, t5741: F, t6844: F, t72: F, t686: F, t4101: F, t6874: F, t10098: F, t10102: F, t10109: F, t10114: F, t14218: F, t14221: F, t14227: F, t14229: F, t14233: F, t14241: F, t14243: F, t22005: F, t5675: F, t5745: F) -> (F, F, F, F, F, F, F, F) {
        let (t22329, t22331, t22332, t22333, t22335, t22336, t22337, t22344) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2109::<F>(t14239, t5741, t6844, t72, t686, t4101, t6874, t10098, t10102, t10109, t10114, t14218, t14221, t14227, t14229, t14233, t14241, t14243, t22005, t5675, t5745);
    (t22329, t22331, t22332, t22333, t22335, t22336, t22337, t22344)
}
