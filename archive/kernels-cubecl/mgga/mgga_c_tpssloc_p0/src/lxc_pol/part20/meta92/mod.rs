//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta92 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk640;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk641;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta92<F: Float>(t666: F, t2331: F, t614: F, t94: F, tau0: F, t659: F, t2248: F, t95: F, t102: F) -> (F, F, F, F, F, F, F, F) {
        let (t2332, t2333, t2336, t2341) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk640::<F>(t666, t2331, t614, t94, tau0);
        let (t2342, t2343, t2346, t2349) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk641::<F>(t659, t2341, t2248, t95, t102);
    (t2332, t2333, t2336, t2341, t2342, t2343, t2346, t2349)
}
