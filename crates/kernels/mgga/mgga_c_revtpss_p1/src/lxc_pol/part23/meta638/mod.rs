//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta638 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2341;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2342;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2343;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta638<F: Float>(t211: F, t9644: F, t209: F, t234: F, t251: F, t268: F, t8779: F, t39497: F, t874: F, t875: F, t10535: F, t231: F, t281: F, t624: F, t836: F, t10529: F, t2453: F, t253: F, t39552: F, t2783: F, t9646: F, t22: F, t837: F, t10111: F, t2789: F, t588: F, t870: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t39644 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2341::<F>(t211, t9644, t209);
        let (t39649, t39652, t39673) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2342::<F>(t234, t251, t268, t39644, t8779, t39497, t874, t875, t10535, t231, t281, t624, t836);
        let (t39680, t39697, t39698, t39701, t39719, t39723) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2343::<F>(t10529, t2453, t253, t39552, t2783, t9646, t22, t251, t837, t10111, t2789, t588, t870);
    (t39644, t39649, t39652, t39673, t39680, t39697, t39698, t39701, t39719, t39723)
}
