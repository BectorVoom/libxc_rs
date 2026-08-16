//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta225 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1277;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1278;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1279;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1280;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1281;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1282;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta225<F: Float>(t2374: F, t9467: F, t702: F, t9454: F, t2411: F, t2409: F, t681: F, t125: F, t141: F, t2413: F, t2508: F, t738: F, t2369: F, t745: F, t180: F, t2511: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t9469, t9474, t9476) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1277::<F>(t2374, t9467, t702, t9454, t2411);
        let (t9478, t9479, t9481, t9482, t9484) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1278::<F>(t2409, t681, t125, t141, t2413, t9454);
        let t9489 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1279::<F>(t2508, t738);
        let t9490 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1280::<F>(t2369, t745);
        let t9493 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1281::<F>(t180, t2511);
        let t9494 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1282::<F>(t9489, t9490, t9493);
    (t9469, t9474, t9476, t9478, t9479, t9481, t9482, t9484, t9489, t9490, t9493, t9494)
}
