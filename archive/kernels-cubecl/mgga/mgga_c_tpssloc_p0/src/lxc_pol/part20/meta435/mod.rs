//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta435 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1863;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1864;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1865;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta435<F: Float>(t1088: F, t14706: F, t123: F, t4778: F, t699: F, t1113: F, t136: F, t4725: F, t690: F, t4730: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t14707, t14708) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1863::<F>(t1088, t14706, t123);
        let (t14710, t14711, t14712, t14713, t14720) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1864::<F>(t4778, t699, t1113, t14706, t136, t4725, t690);
        let (t14721, t14722) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1865::<F>(t14720, t4730, t690);
    (t14707, t14708, t14710, t14711, t14712, t14713, t14720, t14721, t14722)
}
