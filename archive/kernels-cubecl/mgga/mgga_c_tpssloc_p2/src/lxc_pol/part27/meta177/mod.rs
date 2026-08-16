//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta177 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk920;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk921;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk922;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk923;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk924;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk925;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk926;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta177<F: Float>(t1285: F, t588: F, t1287: F, t2423: F, t3686: F, t3697: F, t3819: F, t3821: F, t3823: F, t3825: F, t3828: F, t3830: F, t3832: F, t225: F, t3817: F, t1365: F, t68: F, t3734: F, t1347: F, t3719: F, t1345: F, t1348: F, t546: F, t548: F, t550: F, t1343: F, t820: F, t3791: F, t248: F, t2691: F, t557: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3833, t3834, t3836, t3837) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk920::<F>(t1285, t588, t1287, t2423, t3686, t3697, t3819, t3821, t3823, t3825, t3828, t3830, t3832);
        let (t3839, t3844, t3847, t3850) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk921::<F>(t225, t3817, t3837, t1365, t68, t3734, t1347, t3719, t1345, t1348, t546, t548);
        let t3851 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk922::<F>(t3850, t550);
        let t3853 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk923::<F>(t1343, t3851, t820);
        let t3856 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk924::<F>(t3791, t550);
        let t3858 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk925::<F>(t1343, t3856, t820);
        let t3862 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk926::<F>(t248, t2691, t557);
    (t3833, t3834, t3836, t3839, t3844, t3847, t3850, t3851, t3853, t3856, t3858, t3862)
}
