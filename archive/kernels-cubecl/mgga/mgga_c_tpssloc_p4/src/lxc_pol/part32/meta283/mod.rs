//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta283 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1279;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1280;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1281;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1282;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1283;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1284;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1285;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta283<F: Float>(t1716: F, t2148: F, t1755: F, t7376: F, t7375: F, t1751: F, t2147: F, t462: F, t1734: F, t2144: F, t1246: F, t493: F, t8054: F, t1244: F, t1729: F, t2121: F, t2149: F, t2152: F, t470: F, t7283: F, t7361: F, t7373: F, t7999: F, t8067: F, t1241: F, t1238: F, t1761: F, t2124: F, t2155: F, t4945: F, t498: F, t5055: F, t7282: F, t7351: F, t8003: F, t8006: F, t8011: F, t8015: F, t8018: F, t8055: F, t8061: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t8070 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1279::<F>(t1716, t2148);
        let (t8073, t8074) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1280::<F>(t1755, t7376, t7375);
        let t8077 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1281::<F>(t1751, t2147);
        let (t8078, t8082) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1282::<F>(t462, t8077, t1734, t2144);
        let (t8083, t8085, t8087) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1283::<F>(t1246, t8082, t493, t8054, t1244, t1729, t2121, t2149, t2152, t470, t7283, t7361, t7373, t7999, t8067, t8070, t8074, t8078);
        let t8088 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1284::<F>(t1241, t8087);
        let t8090 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1285::<F>(t1238, t1761, t2121, t2124, t2155, t4945, t498, t5055, t7282, t7283, t7351, t7999, t8003, t8006, t8011, t8015, t8018, t8055, t8061, t8088);
    (t8070, t8073, t8074, t8077, t8078, t8082, t8083, t8085, t8087, t8088, t8090)
}
