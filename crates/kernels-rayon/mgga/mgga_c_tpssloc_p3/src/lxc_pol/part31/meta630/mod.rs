//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta630 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1889;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1890;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta630(t19986: f64, t22833: f64, t5303: f64, t91100: f64, t1339: f64, t550: f64, t56812: f64, t6936: f64, t12289: f64, t1351: f64, t57342: f64, t20473: f64, t3788: f64, t19930: f64, t6952: f64, t1831: f64, t91191: f64, t26257: f64, t5314: f64, t28100: f64, t80853: f64, t80855: f64, t22788: f64, t6431: f64, t6427: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t97320, t97322, t97326, t97333, t97337) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1889(t19986, t22833, t5303, t91100, t1339, t550, t56812, t6936, t12289, t1351, t57342, t20473, t3788);
        let (t97340, t97342, t97344, t97347, t97352, t97354) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1890(t19930, t6952, t1831, t91191, t26257, t5314, t28100, t80853, t80855, t22788, t6431, t6427);
    (t97320, t97322, t97326, t97333, t97337, t97340, t97342, t97344, t97347, t97352, t97354)
}
