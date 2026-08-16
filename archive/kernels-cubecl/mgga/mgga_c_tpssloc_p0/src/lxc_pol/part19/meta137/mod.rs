//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta137 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk714;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk715;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk716;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk717;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta137<F: Float>(t205: F, t3732: F, t1307: F, t210: F, t214: F, t1314: F, t792: F, t118: F, t794: F, t3719: F, t116: F, t534: F, t212: F, t2586: F, t1315: F, t3725: F, t3727: F, t3731: F, t562: F, t1323: F, t1372: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t3733 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk714::<F>(t205, t3732);
        let t3734 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk715::<F>(t1307);
        let (t3736, t3739, t3741, t3742, t3745, t3749) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk716::<F>(t210, t214, t3734, t1314, t792, t118, t1307, t794, t3719, t116, t534, t212);
        let (t3752, t3753, t3755) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk717::<F>(t2586, t3749, t1315, t3725, t3727, t3731, t3733, t3736, t3742, t3745, t562, t1323, t1372);
    (t3733, t3734, t3736, t3739, t3741, t3745, t3749, t3752, t3753, t3755)
}
