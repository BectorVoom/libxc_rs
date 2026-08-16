//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta245 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk981;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk982;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk983;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk984;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta245<F: Float>(t11292: F, t440: F, t11129: F, t3403: F, t11135: F, t11203: F, t11161: F, t11170: F, t11197: F, t11200: F, t11206: F, t11209: F, t11211: F, t11213: F, t11215: F, t11217: F, t11221: F, t11224: F, t11137: F, t11139: F, t11141: F, t11143: F, t11150: F, t11156: F, t11165: F, t11174: F, t11230: F, t11233: F, t11245: F, t11259: F, t11261: F, t11266: F, t1156: F, t1119: F, t3307: F, t3264: F, t1117: F, t3315: F, t3313: F, t1128: F, t3324: F, t1124: F, t3356: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t11365, t11366, t11383) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk981::<F>(t11292, t440, t11129, t3403, t11135, t11203, t11161, t11170, t11197, t11200, t11206, t11209, t11211, t11213, t11215, t11217, t11221, t11224);
        let t11398 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk982::<F>(t11137, t11139, t11141, t11143, t11150, t11156, t11165, t11174, t11230, t11233, t11245, t11259, t11261, t11266);
        let t11399 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk983::<F>(t11383, t11398);
        let (t11400, t11403, t11405, t11407, t11409, t11410, t11415) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk984::<F>(t11399, t1156, t1119, t3307, t3264, t1117, t3315, t3313, t1128, t3324, t1124, t3356);
    (t11365, t11366, t11399, t11400, t11403, t11405, t11407, t11409, t11410, t11415)
}
