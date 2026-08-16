//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta559 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1787;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1788;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta559(t1519: f64, t213: f64, t225: f64, t23168: f64, t25229: f64, t794: f64, t23164: f64, t6555: f64, t7480: f64, t81632: f64, t23030: f64, t25035: f64, t23228: f64, t7479: f64, t81573: f64, t25059: f64, t6562: f64, t82082: f64, t82087: f64, t7488: f64, t82133: f64, t25225: f64, t6547: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t86873, t86886, t86893, t86895, t86903, t86911) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1787(t1519, t213, t225, t23168, t25229, t794, t23164, t6555, t7480, t81632, t23030, t25035);
        let (t86916, t86928, t86930, t86931, t86940, t86942) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1788(t23228, t7479, t81573, t25059, t6562, t794, t82082, t82087, t7488, t82133, t25225, t6547);
    (t86873, t86886, t86893, t86895, t86903, t86911, t86916, t86928, t86930, t86931, t86940, t86942)
}
