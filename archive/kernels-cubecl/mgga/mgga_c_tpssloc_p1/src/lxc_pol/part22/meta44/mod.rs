//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta44 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk318;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk319;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk320;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk321;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk322;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk323;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk324;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta44<F: Float>(t880: F, t154: F, t376: F, t632: F, t607: F, t123: F, t291: F, t287: F, t275: F, t276: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t881, t882) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk318::<F>(t880, t154, t376);
        let t883 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk319::<F>(t632);
        let t884 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk320::<F>(t607, t883);
        let (t885, t886, t888) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk321::<F>(t882, t884, t123, t881);
        let (t890, t891, t892) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk322::<F>(t291, t888, t287);
        let t893 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk323::<F>(t275, t892);
        let t894 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk324::<F>(t276);
    (t881, t882, t883, t884, t885, t886, t888, t890, t891, t892, t893, t894)
}
