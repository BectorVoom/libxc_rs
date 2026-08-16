//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta102 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk660;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk661;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta102<F: Float>(t2331: F, t2332: F, t614: F, t94: F, t659: F, t2248: F, t95: F, t102: F, t662: F, t103: F, t100: F, t657: F, t660: F, t92: F, t96: F, tau0: F, t109: F, t656: F, t2327: F, t2328: F, t64: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2333, t2336, t2341, t2342, t2349, t2350, t2351, t2354, t2355, t2358) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk660::<F>(t2331, t2332, t614, t94, t659, t2248, t95, t102, t662, t103, t100, t657, t660, t92, t96, tau0);
        let (t2359, t2363) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk661::<F>(t109, t2358, t656, t2327, t2328, t2333, t64);
    (t2333, t2336, t2341, t2342, t2349, t2350, t2351, t2354, t2355, t2358, t2359, t2363)
}
