//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta316 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1366;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1367;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta316(t1041: f64, t10870: f64, t3048: f64, t3053: f64, t10478: f64, t3128: f64, t10472: f64, t1015: f64, t1030: f64, t3036: f64, t3033: f64, t248: f64, t3041: f64, t3101: f64, t3039: f64, t3108: f64, t3113: f64, t3121: f64, t1020: f64, t698: f64, t999: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10871, t10873, t10876, t10883, t10889, t10891, t10895) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1366(t1041, t10870, t3048, t3053, t10478, t3128, t10472, t1015, t1030, t3036, t3033, t248, t3041, t3101);
        let (t10896, t10898, t10904, t10909, t10922) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1367(t10895, t3039, t3108, t3113, t10889, t3128, t3033, t248, t3101, t3121, t1020, t698, t999);
    (t10871, t10873, t10876, t10883, t10891, t10896, t10898, t10904, t10909, t10922)
}
