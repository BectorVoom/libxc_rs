//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta184 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk900;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk901;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk902;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk903;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta184<F: Float>(t745: F, t9385: F, t9368: F, t2514: F, t746: F, t2495: F, t744: F, t2576: F, t2582: F, t2584: F, t700: F, t2519: F, t2577: F, t268: F, t2581: F, t675: F, t2585: F, t2565: F, t702: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t9485, t9488, t9501, t9507, t9508, t9514) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk900::<F>(t745, t9385, t9368, t2514, t746, t2495, t744, t2576, t2582, t2584, t700);
        let t9517 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk901::<F>(t2519, t2577, t268);
        let (t9518, t9521) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk902::<F>(t2581, t675, t2585, t268);
        let t9524 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk903::<F>(t2565, t2576, t702);
    (t9485, t9488, t9501, t9507, t9508, t9514, t9517, t9518, t9521, t9524)
}
