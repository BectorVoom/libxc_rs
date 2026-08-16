//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta71 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk499;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk500;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk501;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk502;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk503;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta71<F: Float>(t103: F, t1449: F, t100: F, t104: F, t1445: F, t1447: F, t92: F, t109: F, t656: F, t64: F, t654: F, t510: F, t1409: F, t185: F, t40: F, t52: F, t707: F, t73: F, t76: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
        let (t1450, t1453) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk499::<F>(t103, t1449, t100, t104, t1445, t1447, t92);
        let (t1454, t1458) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk500::<F>(t109, t1453, t656, t64, t654);
        let t1459 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk501::<F>(t1458, t510);
        let t1462 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk502::<F>(t1409, t185);
        let (t1464, t1471) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk503::<F>(t40, t52, t1462, t707, t1409, t73, t76, zeta_threshold);
    (t1450, t1453, t1454, t1458, t1459, t1462, t1464, t1471)
}
