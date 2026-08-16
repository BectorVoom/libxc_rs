//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta323 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1612;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1613;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta323<F: Float>(t13731: F, t2782: F, t212: F, t5710: F, t1358: F, t689: F, t221: F, t3979: F, t5591: F, t3978: F, t3989: F, t5614: F, t5622: F, t9765: F, t1408: F, t240: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t13733, t13734, t13735, t13737, t13760, t13762, t13763) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1612::<F>(t13731, t2782, t212, t5710, t1358, t689, t221, t3979, t5591, t3978, t3989, t5614);
        let (t13765, t13767) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1613::<F>(t5622, t9765, t1408, t240);
    (t13733, t13734, t13735, t13737, t13760, t13762, t13763, t13765, t13767)
}
