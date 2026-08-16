//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta225 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk929;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk930;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta225(t10895: f64, t3039: f64, t3108: f64, t3113: f64, t10889: f64, t3128: f64, t3033: f64, t248: f64, t3101: f64, t3121: f64, t1020: f64, t2250: f64, t607: f64, t4583: f64, t4582: f64, t4588: f64, t698: f64, t999: f64, t973: f64, t2960: f64, t3139: f64, t1000: f64, t1025: f64, t10263: f64, t1041: f64, t1046: f64, t10517: f64, t10860: f64, t10863: f64, t10866: f64, t10871: f64, t10873: f64, t10876: f64, t10879: f64, t10883: f64, t10886: f64, t10891: f64, t3043: f64, t3057: f64, t3109: f64, t3117: f64, t3123: f64, t3134: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10896, t10898, t10903, t10904, t10908, t10909, t10913) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk929(t10895, t3039, t3108, t3113, t10889, t3128, t3033, t248, t3101, t3121, t1020, t2250, t607);
        let (t10914, t10915, t10918, t10919, t10922, t10929) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk930(t10913, t4583, t4582, t4588, t698, t999, t973, t2960, t3139, t1000, t1020, t1025, t10263, t1041, t1046, t10517, t10860, t10863, t10866, t10871, t10873, t10876, t10879, t10883, t10886, t10891, t10896, t10898, t10904, t10909, t3043, t3057, t3109, t3117, t3123, t3134);
    (t10898, t10903, t10904, t10908, t10913, t10914, t10915, t10918, t10919, t10922, t10929)
}
