//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta152 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk762;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk763;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta152<F: Float>(t2229: F, t61: F, t119: F, t212: F, t252: F, t828: F, t1929: F, t343: F, t984: F, t3034: F, t334: F, rho0: F, t371: F, t533: F, t556: F, t1351: F, t562: F, t1388: F, t3701: F, t1184: F, t460: F, t590: F, t60: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t6597, t6600, t6647, t6720, t6733, t6739) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk762::<F>(t2229, t61, t119, t212, t252, t828, t1929, t343, t984, t3034, t334, rho0);
        let (t6793, t6924, t6977, t6999, t7319, t8705) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk763::<F>(t334, t371, t533, t556, t1351, t562, t1388, t3701, t1184, t460, t590, t60);
    (t6597, t6600, t6647, t6720, t6733, t6739, t6793, t6924, t6977, t6999, t7319, t8705)
}
