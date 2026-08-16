//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta103 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk662;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk663;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk664;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk665;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk666;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta103<F: Float>(t2363: F, t510: F, t177: F, t738: F, t745: F, t746: F, t761: F, t118: F, t187: F, t677: F, t763: F, t200: F, t262: F, t776: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t2364, t2368, t2369) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk662::<F>(t2363, t510, t177, t738, t745);
        let t2371 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk663::<F>(t2368, t2369, t746);
        let (t2373, t2374) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk664::<F>(t2371, t761, t118, t187);
        let t2375 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk665::<F>(t677, t763);
        let (t2377, t2378, t2379) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk666::<F>(t2374, t2375, t200, t262, t776);
    (t2364, t2368, t2369, t2371, t2373, t2374, t2375, t2377, t2378, t2379)
}
