//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta34 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk242;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk243;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk244;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk245;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk246;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta34<F: Float>(t120: F, t676: F, t118: F, t142: F, t138: F, t125: F, t126: F, t67: F, t117: F, t123: F, t3: F, t61: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t677 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk242::<F>(t120, t676);
        let t680 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk243::<F>(t118, t142, t677);
        let (t681, t682, t683, t685, t686) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk244::<F>(t138, t125, t126, t67, t117, t120);
        let (t687, t688, t690) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk245::<F>(t676, t686, t685, t118, t677);
        let (t693, t694, t697) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk246::<F>(t123, t67, t687, t3, t61);
    (t677, t680, t681, t682, t683, t685, t686, t688, t690, t693, t694, t697)
}
