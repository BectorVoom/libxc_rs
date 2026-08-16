//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta139 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk721;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk722;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk723;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk724;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk725;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk726;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta139<F: Float>(t1337: F, t551: F, t236: F, t240: F, t1336: F, t1351: F, t550: F, t1343: F, t820: F, t1339: F, t835: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t3787 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk721::<F>(t1337, t551);
        let t3788 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk722::<F>(t236, t3787);
        let (t3789, t3790, t3791) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk723::<F>(t240, t3788, t1336, t1351);
        let t3792 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk724::<F>(t550);
        let (t3793, t3795) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk725::<F>(t3791, t3792, t1343, t820);
        let (t3798, t3799) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk726::<F>(t1339, t835, t1336);
    (t3787, t3788, t3789, t3790, t3791, t3792, t3793, t3795, t3798, t3799)
}
