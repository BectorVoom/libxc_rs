//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta311 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1378;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1379;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta311<F: Float>(t10895: F, t3039: F, t3108: F, t3113: F, t10889: F, t3128: F, t3033: F, t248: F, t3101: F, t3121: F, t1020: F, t698: F, t999: F, t973: F, t2960: F, t3139: F, t1030: F, t363: F, t3068: F, t1058: F, t3030: F, t990: F, t3032: F, t3129: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10896, t10898, t10904, t10908, t10909, t10922) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1378::<F>(t10895, t3039, t3108, t3113, t10889, t3128, t3033, t248, t3101, t3121, t1020, t698, t999);
        let (t10923, t10927, t10937, t10947, t10948, t10949) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1379::<F>(t10922, t973, t2960, t3139, t1030, t363, t3068, t1058, t3030, t990, t3032, t3129);
    (t10896, t10898, t10904, t10908, t10909, t10922, t10923, t10927, t10937, t10947, t10948, t10949)
}
