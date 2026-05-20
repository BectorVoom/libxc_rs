//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta942 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3093;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3094;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3095;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta942<F: Float>(t1196: F, t20891: F, t24375: F, t43752: F, t16840: F, t20574: F, t17092: F, t20577: F, t1149: F, t12248: F, t24221: F, t3433: F, t5104: F, t6439: F, t12361: F, t24212: F, t3384: F, t5105: F, t6470: F, t24765: F, t3531: F, t16988: F, t20472: F, t1733: F, t20447: F, t12243: F, t24215: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t81589, t81591, t81593, t81596, t81599) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3093::<F>(t1196, t20891, t24375, t43752, t16840, t20574, t17092, t20577, t1149, t12248, t24221, t3433, t5104, t6439);
        let (t81601, t81604, t81606, t81609, t81612) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3094::<F>(t12361, t24212, t3384, t5105, t6470, t24765, t3531, t1196, t16988, t20472, t1733, t20447);
        let (t81614, t81615) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3095::<F>(t12243, t24215, t81589, t81591, t81593, t81596, t81599, t81601, t81604, t81606, t81609, t81612);
    (t81589, t81591, t81593, t81596, t81599, t81601, t81604, t81606, t81609, t81612, t81614, t81615)
}
