//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta55 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk371;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk372;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk373;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk374;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk375;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk376;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta55<F: Float>(t1059: F, t1060: F, t1049: F, t383: F, t1003: F, t1058: F, t353: F, t384: F, t1055: F, t1050: F, t1052: F, t388: F, t991: F, t390: F, t25: F, t265: F, t394: F, t193: F, t336: F, t873: F, t890: F, t916: F, t956: F, t958: F, t963: F, t396: F, t40: F, t606: F, t607: F, dens_threshold: F, rho0: F, zeta_threshold: F, t268: F, t405: F, t878: F, t154: F, t486: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1061, t1063, t1065, t1066) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk371::<F>(t1059, t1060, t1049, t383, t1003, t1058, t353, t384, t1055);
        let (t1068, t1070) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk372::<F>(t1050, t1052, t1066, t388, t991, t390);
        let (t1074, t1079) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk373::<F>(t25, t265, t394, t1068, t1070, t193, t336, t873, t890, t916, t956, t958, t963, t396, t40, t606, t607, dens_threshold, rho0, zeta_threshold);
        let t1081 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk374::<F>(t606);
        let t1086 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk375::<F>(t268, t405, t878);
        let (t1087, t1088) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk376::<F>(t1086, t154, t486);
    (t1061, t1063, t1065, t1066, t1068, t1070, t1074, t1079, t1081, t1086, t1087, t1088)
}
