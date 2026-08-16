//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta571 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1804;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1805;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta571<F: Float>(t23185: F, t4283: F, t81914: F, t25300: F, t81591: F, t81633: F, t25303: F, t6579: F, t23110: F, t4292: F, t25288: F, t234: F, t4265: F, t25237: F, t23168: F, t25307: F, t25287: F, t81651: F, t22893: F, t23164: F, t25320: F, t7521: F, t81632: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t87544, t87546, t87559, t87565, t87581, t87583, t87586) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1804::<F>(t23185, t4283, t81914, t25300, t81591, t81633, t25303, t6579, t23110, t4292, t25288, t234, t4265);
        let (t87601, t87603, t87612, t87618, t87635) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1805::<F>(t23110, t23185, t25237, t23168, t25307, t25287, t81651, t22893, t23164, t25320, t7521, t81632);
    (t87544, t87546, t87559, t87565, t87581, t87583, t87586, t87601, t87603, t87612, t87618, t87635)
}
