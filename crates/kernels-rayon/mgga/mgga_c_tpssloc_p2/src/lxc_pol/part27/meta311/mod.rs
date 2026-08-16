//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta311 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1378;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1379;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta311(t10895: f64, t3039: f64, t3108: f64, t3113: f64, t10889: f64, t3128: f64, t3033: f64, t248: f64, t3101: f64, t3121: f64, t1020: f64, t698: f64, t999: f64, t973: f64, t2960: f64, t3139: f64, t1030: f64, t363: f64, t3068: f64, t1058: f64, t3030: f64, t990: f64, t3032: f64, t3129: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10896, t10898, t10904, t10908, t10909, t10922) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1378(t10895, t3039, t3108, t3113, t10889, t3128, t3033, t248, t3101, t3121, t1020, t698, t999);
        let (t10923, t10927, t10937, t10947, t10948, t10949) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1379(t10922, t973, t2960, t3139, t1030, t363, t3068, t1058, t3030, t990, t3032, t3129);
    (t10896, t10898, t10904, t10908, t10909, t10922, t10923, t10927, t10937, t10947, t10948, t10949)
}
