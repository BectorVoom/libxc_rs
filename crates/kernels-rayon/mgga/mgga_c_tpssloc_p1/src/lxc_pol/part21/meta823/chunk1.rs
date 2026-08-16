//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2894/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2894(t17520: f64, t2836: f64, t2842: f64, t10704: f64, t5726: f64, t10702: f64, t2793: f64, t13654: f64, t4399: f64, t17527: f64, t42100: f64, t42102: f64, t5694: f64) -> (f64, f64, f64, f64, f64) {
    let t60377 = 0.16081979498692535067e2_f64 * t2842 * t17520 * t2836;
    let t60378 = t5726 * t10704;
    let t60381 = 0.51726012919273400301e3_f64 * t10702 * t60378 * t2793;
    let t60384 = 0.32163958997385070134e2_f64 * t2842 * t4399 * t13654;
    let t60387 = 0.51726012919273400301e3_f64 * t10702 * t17527 * t2836;
    let t60391 = 0.24955700379505800916e5_f64 * t42100 * t5694 * t42102 * t2793;
    (t60377, t60381, t60384, t60387, t60391)
}
