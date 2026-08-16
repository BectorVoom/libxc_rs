//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta89 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk625;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk626;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk627;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk628;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk629;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta89<F: Float>(t2248: F, t25: F, t28: F, zeta_threshold: F, t31: F, t65: F, t608: F, t628: F, t36: F, t365: F, sigma0: F, t42: F, t2244: F, t43: F, t54: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t2249 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk625::<F>(t2248);
        let t2250 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk626::<F>(t25, t28, t2249, zeta_threshold);
        let (t2251, t2252, t2255, t2261, t2262) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk627::<F>(t2250, t31, t65, t608, t628, t36, t365, sigma0);
        let t2267 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk628::<F>(t42);
        let (t2268, t2271, t2274) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk629::<F>(t2244, t2267, t2250, t43, t54);
    (t2249, t2250, t2251, t2252, t2255, t2261, t2262, t2267, t2268, t2271, t2274)
}
