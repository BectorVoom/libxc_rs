//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta120 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk610;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk611;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk612;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk613;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta120<F: Float>(t1213: F, t4997: F, t1009: F, t1720: F, t1011: F, t1212: F, t1226: F, t1730: F, t1017: F, t1742: F, t1210: F, t1207: F, t372: F, t479: F, t471: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t4998, t5000, t5001, t5002) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk610::<F>(t1213, t4997, t1009, t1720, t1011, t1212);
        let t5005 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk611::<F>(t1226, t1730);
        let (t5018, t5019) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk612::<F>(t1017, t1742, t1210, t1207);
        let (t5023, t5024) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk613::<F>(t1742, t372, t479, t471);
    (t4998, t5000, t5001, t5002, t5005, t5018, t5019, t5023, t5024)
}
