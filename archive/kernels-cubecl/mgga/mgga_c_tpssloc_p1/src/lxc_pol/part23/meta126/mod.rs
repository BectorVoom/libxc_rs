//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta126 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk624;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk625;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk626;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk627;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta126<F: Float>(t3787: F, t68: F, t544: F, t1824: F, t562: F, t1338: F, t1834: F, t112: F, t1851: F, t2218: F, t2220: F, t2222: F, t2224: F, t2226: F, t2228: F, t2232: F, t1437: F, t1409: F, t65: F, t11: F, t2219: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t5333, t5334, t5335, t5343, t5344, t5348, t5371, t5385) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk624::<F>(t3787, t68, t544, t1824, t562, t1338, t1834, t112, t1851, t2218, t2220, t2222, t2224, t2226, t2228, t2232);
        let t5389 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk625::<F>(t1437);
        let t5392 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk626::<F>(t1409);
        let (t5393, t5396) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk627::<F>(t5392, t65, t11, t2219);
    (t5333, t5334, t5335, t5343, t5344, t5348, t5371, t5385, t5389, t5392, t5393, t5396)
}
