//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta55 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk382;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk383;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk384;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk385;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk386;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk387;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk388;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta55(t1049: f64, t349: f64, t225: f64, t382: f64, t386: f64, t68: f64, t1011: f64, t1014: f64, t1010: f64, t1022: f64, t381: f64, t357: f64, t360: f64, t383: f64, t1003: f64, t353: f64, t384: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1050, t1052) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk382(t1049, t349, t225, t382);
        let (t1053, t1054) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk383(t386);
        let t1055 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk384(t1054, t68);
        let t1057 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk385(t1011, t1014);
        let t1058 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk386(t1010, t1057);
        let (t1059, t1060) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk387(t1022, t381, t357, t360);
        let (t1061, t1063, t1065) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk388(t1059, t1060, t1049, t383, t1003, t1058, t353, t384);
    (t1050, t1052, t1053, t1054, t1055, t1057, t1058, t1059, t1060, t1061, t1063, t1065)
}
