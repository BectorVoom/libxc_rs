//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta44 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk295;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk296;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk297;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk298;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk299;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk300;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta44<F: Float>(t1017: F, t368: F, t1015: F, t1012: F, t376: F, t61: F, t122: F, t374: F, t370: F, t372: F, t364: F, t354: F, t270: F, t283: F, t225: F, t382: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t1019, t1020) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk295::<F>(t1017, t368, t1015, t1012);
        let t1021 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk296::<F>(t376, t61);
        let (t1036, t1038, t1040, t1041) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk297::<F>(t122, t374, t376, t370, t368, t372, t364, t354);
        let t1043 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk298::<F>(t270, t283);
        let t1044 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk299::<F>(t1043, t61);
        let t1052 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk300::<F>(t225, t382);
    (t1019, t1020, t1021, t1036, t1038, t1040, t1041, t1043, t1044, t1052)
}
