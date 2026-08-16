//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta245 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1106;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1107;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1108;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1109;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta245<F: Float>(t625: F, t44: F, t607: F, t614: F, t6500: F, t67: F, t1864: F, t641: F, t71: F, t1863: F, t5: F, t1860: F, t1865: F, t6486: F, t6490: F, t6492: F, t6495: F, t112: F, t111: F, t1868: F, t1874: F, t2314: F, t4034: F, t1266: F, t1873: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t6503, t6504, t6505, t6506) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1106::<F>(t625, t44, t607, t614, t6500, t67, t1864);
        let (t6509, t6510) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1107::<F>(t641, t71, t1863);
        let (t6514, t6515, t6517) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1108::<F>(t5, t1860, t1865, t6486, t6490, t6492, t6495, t6506, t6510, t112, t111, t1868);
        let (t6522, t6524, t6525) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1109::<F>(t1874, t2314, t4034, t1266, t1873);
    (t6503, t6504, t6505, t6506, t6509, t6510, t6514, t6515, t6517, t6522, t6524, t6525)
}
