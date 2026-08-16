//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta53 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk374;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk375;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk376;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk377;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk378;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk379;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk380;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk381;
use chunk8::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk382;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta53(t1010: f64, t1057: f64, t1022: f64, t381: f64, t357: f64, t360: f64, t1049: f64, t383: f64, t1003: f64, t353: f64, t384: f64, t1055: f64, t1050: f64, t1052: f64, t388: f64, t991: f64, t390: f64, t25: f64, t265: f64, t394: f64, t193: f64, t336: f64, t873: f64, t890: f64, t916: f64, t956: f64, t958: f64, t963: f64, t396: f64, t40: f64, t606: f64, t607: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t268: f64, t405: f64, t878: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1058 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk374(t1010, t1057);
        let (t1059, t1060) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk375(t1022, t381, t357, t360);
        let (t1061, t1063, t1065) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk376(t1059, t1060, t1049, t383, t1003, t1058, t353, t384);
        let t1066 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk377(t1055, t1065);
        let t1068 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk378(t1050, t1052, t1066, t388, t991);
        let t1070 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk379(t390);
        let (t1074, t1079) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk380(t25, t265, t394, t1068, t1070, t193, t336, t873, t890, t916, t956, t958, t963, t396, t40, t606, t607, dens_threshold, rho0, zeta_threshold);
        let t1081 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk381(t606);
        let t1086 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk382(t268, t405, t878);
    (t1058, t1060, t1061, t1063, t1065, t1066, t1068, t1070, t1074, t1079, t1081, t1086)
}
