//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta943 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3096;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3097;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta943<F: Float>(t1732: F, t3433: F, t69591: F, t20644: F, t5104: F, t5068: F, t68792: F, t5109: F, t68952: F, t17092: F, t20641: F, t16840: F, t20645: F, t20580: F, t58342: F, t20648: F, t20652: F, t58473: F, t1149: F, t12227: F, t24262: F, t12248: F, t6474: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t81618, t81621, t81623, t81625, t81627, t81629) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3096::<F>(t1732, t3433, t69591, t20644, t5104, t5068, t68792, t5109, t68952, t17092, t20641, t16840, t20645);
        let (t81631, t81633, t81635, t81638, t81641) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3097::<F>(t20580, t58342, t16840, t20648, t20652, t58473, t1149, t12227, t24262, t12248, t5104, t6474);
    (t81618, t81621, t81623, t81625, t81627, t81629, t81631, t81633, t81635, t81638, t81641)
}
