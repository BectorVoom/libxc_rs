//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta54 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk354;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk355;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk356;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk357;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk358;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk359;
use chunk6::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk360;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta54<F: Float>(t1030: F, t364: F, t354: F, t122: F, t374: F, t376: F, t370: F, t368: F, t372: F, t270: F, t283: F, t61: F, t248: F, t884: F, t1000: F, t1005: F, t1020: F, t1025: F, t350: F, t378: F, t964: F, t973: F, t997: F, t349: F, t225: F, t382: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1031, t1032, t1036) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk354::<F>(t1030, t364, t354, t122, t374, t376);
        let (t1038, t1040) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk355::<F>(t1036, t370, t368, t372, t364);
        let t1041 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk356::<F>(t1040, t354);
        let t1043 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk357::<F>(t270, t283);
        let t1044 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk358::<F>(t1043, t61);
        let (t1046, t1049) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk359::<F>(t1044, t248, t884, t1000, t1005, t1020, t1025, t1032, t1038, t1041, t350, t378, t964, t973, t997);
        let (t1050, t1052) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk360::<F>(t1049, t349, t225, t382);
    (t1031, t1032, t1036, t1038, t1040, t1041, t1043, t1044, t1046, t1049, t1050, t1052)
}
