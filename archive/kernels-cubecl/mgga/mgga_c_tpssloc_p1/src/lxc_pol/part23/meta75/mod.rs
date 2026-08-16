//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta75 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk442;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk443;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk444;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk445;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta75<F: Float>(t2368: F, t2369: F, t746: F, t761: F, t118: F, t187: F, t677: F, t763: F, t200: F, t262: F, t123: F, t126: F, t131: F, t119: F, t132: F, t63: F, t204: F, t686: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t2371 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk442::<F>(t2368, t2369, t746);
        let (t2373, t2374) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk443::<F>(t2371, t761, t118, t187);
        let t2375 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk444::<F>(t677, t763);
        let (t2377, t2378, t2385, t2386, t2387, t2388, t2390) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk445::<F>(t2374, t2375, t200, t262, t123, t126, t131, t119, t132, t63, t204, t686);
    (t2371, t2373, t2374, t2375, t2377, t2378, t2385, t2386, t2387, t2388, t2390)
}
