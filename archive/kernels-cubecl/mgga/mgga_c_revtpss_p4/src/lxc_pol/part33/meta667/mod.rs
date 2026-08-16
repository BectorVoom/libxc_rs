//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta667 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2191;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2192;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta667<F: Float>(t1907: F, t5591: F, t25082: F, t8717: F, t29495: F, t7235: F, t5778: F, t28196: F, t28197: F, t28184: F, t7898: F, t5920: F, t648: F, t1937: F, t21881: F, t94: F, t29508: F, t6993: F, t86815: F, t7003: F, t27123: F, t7735: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t108685, t108687, t108691, t108693, t108710) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2191::<F>(t1907, t5591, t25082, t8717, t29495, t7235, t5778, t28196, t28197, t28184, t7898, t5920, t648);
        let (t108712, t108716, t108718, t108721, t108723, t108725) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2192::<F>(t108710, t1937, t21881, t94, t29508, t6993, t25082, t86815, t8717, t7003, t27123, t7735);
    (t108685, t108687, t108691, t108693, t108710, t108712, t108716, t108718, t108721, t108723, t108725)
}
