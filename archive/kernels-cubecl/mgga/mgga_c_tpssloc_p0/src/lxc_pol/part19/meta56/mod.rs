//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta56 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk363;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk364;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk365;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk366;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk367;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk368;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta56<F: Float>(t432: F, t427: F, t1086: F, t1111: F, t1092: F, t1103: F, t1108: F, t1115: F, t435: F, t449: F, t445: F, t440: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1127, t1128, t1129, t1136) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk363::<F>(t432, t427, t1086, t1111, t1092, t1103, t1108, t1115);
        let t1137 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk364::<F>(t435);
        let t1138 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk365::<F>(t1136, t1137);
        let t1143 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk366::<F>(t1086, t1092);
        let (t1144, t1146, t1147) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk367::<F>(t1143, t449, t445);
        let (t1148, t1155) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk368::<F>(t1147, t440, t1086, t1111, t1092, t1103, t1108, t1115);
    (t1127, t1128, t1129, t1136, t1137, t1138, t1143, t1144, t1146, t1147, t1148, t1155)
}
