//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta327 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1697;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1698;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1699;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1700;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta327<F: Float>(t1307: F, t212: F, t12225: F, t2586: F, t535: F, t9534: F, t9538: F, t3792: F, t3850: F, t1337: F, t550: F, t1338: F, t3879: F, t3773: F, t68: F, t1339: F, t836: F, t1336: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t12227, t12228, t12236, t12240) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1697::<F>(t1307, t212, t12225, t2586, t535, t9534, t9538, t3792, t3850);
        let (t12247, t12248, t12250) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1698::<F>(t1337, t3792, t550);
        let (t12259, t12267) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1699::<F>(t1338, t3879, t3773, t68);
        let (t12282, t12283) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1700::<F>(t1339, t836, t1336);
    (t12227, t12228, t12236, t12240, t12247, t12248, t12250, t12259, t12267, t12282, t12283)
}
