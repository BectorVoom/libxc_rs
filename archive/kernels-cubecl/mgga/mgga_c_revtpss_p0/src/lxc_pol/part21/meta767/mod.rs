//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta767 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2719;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2720;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta767<F: Float>(t50014: F, t50033: F, t162: F, t187: F, t40092: F, t40094: F, t14365: F, t14397: F, t2403: F, t39818: F, t39823: F, t40084: F, t40088: F, t49992: F, t49994: F, t49995: F, t14383: F, t2398: F, t40108: F, t14616: F, t2619: F, t40207: F, t4403: F, t40119: F, t40121: F, t14386: F, t2615: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t50034, t50037, t50038, t50039, t50040) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2719::<F>(t50014, t50033, t162, t187, t40092, t40094, t14365, t14397, t2403, t39818, t39823, t40084, t40088, t49992, t49994, t49995);
        let (t50045, t50046, t50048, t50051, t50055, t50056, t50058) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2720::<F>(t14383, t2398, t40108, t14616, t2619, t162, t40207, t4403, t40119, t40121, t14386, t2615);
    (t50034, t50037, t50038, t50039, t50040, t50045, t50046, t50048, t50051, t50055, t50056, t50058)
}
