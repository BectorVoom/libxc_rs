//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta60 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk411;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk412;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk413;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk414;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk415;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk416;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta60<F: Float>(t1136: F, t1137: F, t1086: F, t1092: F, t449: F, t445: F, t440: F, t1111: F, t1103: F, t1108: F, t1115: F, t448: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1138, t1141, t1143) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk411::<F>(t1136, t1137, t1086, t1092);
        let (t1144, t1146, t1147) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk412::<F>(t1143, t449, t445);
        let t1148 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk413::<F>(t1147, t440);
        let (t1150, t1153, t1155) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk414::<F>(t1086, t1111, t1092, t1103, t1108, t1115);
        let t1156 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk415::<F>(t448);
        let t1157 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk416::<F>(t1155, t1156);
    (t1138, t1141, t1143, t1144, t1146, t1147, t1148, t1150, t1153, t1155, t1156, t1157)
}
