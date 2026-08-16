//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta309 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1559;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1560;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1561;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta309<F: Float>(t11129: F, t11292: F, t3403: F, t1164: F, t1143: F, t3375: F, t1156: F, t1124: F, t3331: F, t1136: F, t3333: F, t1137: F, t11282: F, t440: F, t11285: F, t11135: F, t11203: F, t11161: F, t11170: F, t11197: F, t11200: F, t11206: F, t11209: F, t11211: F, t11213: F, t11215: F, t11217: F, t11221: F, t11224: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t11294, t11296, t11297, t11300, t11303, t11306) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1559::<F>(t11129, t11292, t3403, t1164, t1143, t3375, t1156, t1124, t3331, t1136, t3333);
        let (t11307, t11310) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1560::<F>(t11306, t1137, t11282, t440);
        let (t11311, t11314, t11317, t11328) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1561::<F>(t11129, t11285, t11135, t11203, t11161, t11170, t11197, t11200, t11206, t11209, t11211, t11213, t11215, t11217, t11221, t11224);
    (t11294, t11296, t11297, t11300, t11303, t11306, t11307, t11310, t11311, t11314, t11317, t11328)
}
