//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta56 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk386;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk387;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk388;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk389;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk390;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk391;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta56<F: Float>(t25: F, t265: F, t394: F, t1068: F, t1070: F, t193: F, t336: F, t873: F, t890: F, t916: F, t956: F, t958: F, t963: F, t396: F, t40: F, t606: F, t607: F, dens_threshold: F, rho0: F, zeta_threshold: F, t268: F, t405: F, t878: F, t154: F, t486: F, t636: F) -> (F, F, F, F, F, F, F, F) {
        let (t1074, t1079) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk386::<F>(t25, t265, t394, t1068, t1070, t193, t336, t873, t890, t916, t956, t958, t963, t396, t40, t606, t607, dens_threshold, rho0, zeta_threshold);
        let t1081 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk387::<F>(t606);
        let t1086 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk388::<F>(t268, t405, t878);
        let (t1087, t1088) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk389::<F>(t1086, t154, t486);
        let t1089 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk390::<F>(t636);
        let t1090 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk391::<F>(t1089, t607);
    (t1074, t1079, t1081, t1086, t1087, t1088, t1089, t1090)
}
