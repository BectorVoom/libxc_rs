//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta460 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1739;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1740;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1741;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1742;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta460<F: Float>(t234: F, t852: F, t117: F, t229: F, t67: F, t6559: F, t22893: F, t6639: F, t6546: F, t6551: F, t6640: F, t22641: F, t2587: F, t22690: F, t6638: F, t206: F, t268: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t23153, t23163, t23164) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1739::<F>(t234, t852, t117, t229, t67, t6559);
        let (t23165, t23166, t23168) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1740::<F>(t22893, t6639, t23164, t6546, t6551);
        let (t23169, t23171) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1741::<F>(t23168, t6640, t22641, t2587);
        let (t23172, t23174, t23185) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1742::<F>(t22690, t6638, t23171, t206, t268, t6559);
    (t23153, t23163, t23164, t23165, t23166, t23168, t23169, t23171, t23172, t23174, t23185)
}
