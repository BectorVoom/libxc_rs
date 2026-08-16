//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2917/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2917(t300: f64, t59928: f64, t59982: f64, t60030: f64, t60346: f64, t60401: f64, t60711: f64, t60763: f64, t60806: f64, t17955: f64, t2940: f64, t17930: f64) -> (f64, f64, f64) {
    let t60810 = t300 * (t59928 + t59982 + t60030 + t60346 + t60401 + t60711 + t60763 + t60806);
    let t60812 = 0.34631718211362927518e2_f64 * t2940 * t17955;
    let t60814 = 0.69263436422725855036e2_f64 * t2940 * t17930;
    (t60810, t60812, t60814)
}
