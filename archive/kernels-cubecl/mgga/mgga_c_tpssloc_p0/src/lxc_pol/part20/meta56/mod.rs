//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta56 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk410;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk411;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk412;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk413;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk414;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk415;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta56<F: Float>(t1100: F, t1102: F, t1086: F, t407: F, t281: F, t415: F, t904: F, t241: F, t457: F, t1090: F, t136: F, t1092: F, t422: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1103, t1105, t1107) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk410::<F>(t1100, t1102, t1086, t407);
        let (t1108, t1111) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk411::<F>(t1102, t1107, t281, t415, t904);
        let (t1112, t1113) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk412::<F>(t1111, t241, t457);
        let (t1114, t1115, t1117) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk413::<F>(t1090, t1113, t136, t1092, t1103, t1105, t1108, t1112);
        let t1118 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk414::<F>(t422);
        let t1119 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk415::<F>(t1117, t1118);
    (t1103, t1105, t1107, t1108, t1111, t1112, t1113, t1114, t1115, t1117, t1118, t1119)
}
