//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta394 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1308;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1309;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1310;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta394<F: Float>(t258: F, t39552: F, t2454: F, t2455: F, t39494: F, t14545: F, t251: F, t786: F, t2710: F, t2793: F, t211: F, t9644: F, t209: F, t234: F, t268: F, t8779: F, t39497: F, t874: F, t875: F, t10529: F, t2453: F, t253: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t39554, t39557, t39598, t39633, t39643) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1308::<F>(t258, t39552, t2454, t2455, t39494, t14545, t251, t786, t2710, t2793, t211, t9644);
        let t39644 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1309::<F>(t209, t39643);
        let (t39649, t39652, t39680, t39697) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1310::<F>(t234, t251, t268, t39644, t8779, t39497, t874, t875, t10529, t2453, t253, t39552);
    (t39554, t39557, t39598, t39633, t39644, t39649, t39652, t39680, t39697)
}
