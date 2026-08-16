//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta681 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2245;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2246;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta681(t10937: f64, t18041: f64, t1041: f64, t13969: f64, t17636: f64, t17642: f64, t17906: f64, t3117: f64, t17624: f64, t2960: f64, t5884: f64, t698: f64, t973: f64, t5889: f64, t10422: f64, t17676: f64, t3070: f64, t17171: f64, t2970: f64, t17167: f64, t10231: f64, t17157: f64, t17161: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t62499, t62510, t62515, t62534, t62556, t62559) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2245(t10937, t18041, t1041, t13969, t17636, t17642, t17906, t3117, t17624, t2960, t5884, t698, t973);
        let (t62565, t62602, t62631, t62640, t62657, t62660) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2246(t5889, t698, t973, t10422, t17676, t3070, t17171, t2970, t17167, t10231, t17157, t17161);
    (t62499, t62510, t62515, t62534, t62556, t62559, t62565, t62602, t62631, t62640, t62657, t62660)
}
