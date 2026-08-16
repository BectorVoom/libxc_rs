//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta91 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk636;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk637;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk638;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk639;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta91<F: Float>(t5: F, t2233: F, t2235: F, t2240: F, t2241: F, t2307: F, t605: F, t645: F, t86: F, t112: F, t111: F, t649: F, t671: F, t89: F, t1266: F, t107: F, t2281: F, t626: F, t667: F, t106: F, t655: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t2311, t2312) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk636::<F>(t5, t2233, t2235, t2240, t2241, t2307, t605, t645, t86, t112);
        let t2314 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk637::<F>(t111, t649);
        let t2319 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk638::<F>(t671);
        let (t2320, t2323, t2327, t2328, t2331) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk639::<F>(t2319, t89, t1266, t671, t107, t2281, t626, t667, t106, t655);
    (t2311, t2312, t2314, t2319, t2320, t2323, t2327, t2328, t2331)
}
