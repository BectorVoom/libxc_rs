//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta779 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2584;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2585;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta779<F: Float>(t45619: F, t58919: F, t3666: F, t5390: F, t43766: F, t44361: F, t45608: F, t45786: F, t12984: F, t5323: F, t17500: F, t372: F, t13142: F, t56878: F, t12851: F, t1778: F, t3766: F, t5219: F, t5330: F, t3718: F, t44546: F, t5353: F, t45833: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t58920, t58927, t58983, t59001, t59011, t59041, t59062) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2584::<F>(t45619, t58919, t3666, t5390, t43766, t44361, t45608, t45786, t12984, t5323, t17500, t372);
        let (t59066, t59144, t59162, t59186, t59196) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2585::<F>(t13142, t56878, t12851, t1778, t3766, t5219, t5330, t3718, t44546, t5353, t45833, t58919);
    (t58920, t58927, t58983, t59001, t59011, t59041, t59062, t59066, t59144, t59162, t59186, t59196)
}
