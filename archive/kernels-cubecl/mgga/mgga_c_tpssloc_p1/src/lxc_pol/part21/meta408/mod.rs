//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta408 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1903;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1904;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1905;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1906;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta408<F: Float>(t14704: F, t1089: F, t12606: F, t1088: F, t123: F, t4778: F, t699: F, t1113: F, t136: F, t4725: F, t690: F, t4730: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t14705, t14706) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1903::<F>(t14704, t1089, t12606);
        let (t14707, t14708) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1904::<F>(t1088, t14706, t123);
        let (t14710, t14711, t14712, t14713, t14720) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1905::<F>(t4778, t699, t1113, t14706, t136, t4725, t690);
        let (t14721, t14722) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1906::<F>(t14720, t4730, t690);
    (t14705, t14706, t14707, t14708, t14710, t14711, t14712, t14713, t14720, t14721, t14722)
}
