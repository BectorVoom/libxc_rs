//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta280 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1291;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1292;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1293;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1294;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1295;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta280(t1653: f64, t7363: f64, t7362: f64, t1716: f64, t2148: f64, t1755: f64, t7376: f64, t7375: f64, t1751: f64, t2147: f64, t462: f64, t1734: f64, t2144: f64, t1246: f64, t493: f64, t8054: f64, t1244: f64, t1729: f64, t2121: f64, t2149: f64, t2152: f64, t470: f64, t7283: f64, t7361: f64, t7373: f64, t7999: f64, t1241: f64, t1238: f64, t1761: f64, t2124: f64, t2155: f64, t4945: f64, t498: f64, t5055: f64, t7282: f64, t7351: f64, t8003: f64, t8006: f64, t8011: f64, t8015: f64, t8018: f64, t8055: f64, t8061: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t8066, t8067, t8070, t8073, t8074, t8077) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1291(t1653, t7363, t7362, t1716, t2148, t1755, t7376, t7375, t1751, t2147);
        let (t8078, t8082) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1292(t462, t8077, t1734, t2144);
        let (t8083, t8085, t8087) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1293(t1246, t8082, t493, t8054, t1244, t1729, t2121, t2149, t2152, t470, t7283, t7361, t7373, t7999, t8067, t8070, t8074, t8078);
        let t8088 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1294(t1241, t8087);
        let t8090 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1295(t1238, t1761, t2121, t2124, t2155, t4945, t498, t5055, t7282, t7283, t7351, t7999, t8003, t8006, t8011, t8015, t8018, t8055, t8061, t8088);
    (t8066, t8067, t8070, t8073, t8074, t8077, t8082, t8083, t8085, t8087, t8088, t8090)
}
