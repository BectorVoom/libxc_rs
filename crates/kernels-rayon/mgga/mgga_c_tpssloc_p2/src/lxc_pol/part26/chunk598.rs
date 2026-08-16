//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 598/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk598(t300: f64, t3407: f64, t3369: f64, t1143: f64, t1166: f64, t1156: f64, t3375: f64, t3377: f64, t1164: f64, t1147: f64, t3395: f64, t3400: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3408 = t300 * t3407;
    let t3410 = 0.19751673498613801407e-1_f64 * t300 * t3369;
    let t3411 = t300 * t1143;
    let t3413 = 0.11696447245269292414e1_f64 * t3411 * t1166;
    let t3415 = t3375 * t3377 * t1156;
    let t3417 = 0.11696447245269292414e1_f64 * t1164 * t3415;
    let t3419 = t1147 * t3395 * t1156;
    let t3421 = 0.5848223622634646207e0_f64 * t1164 * t3419;
    let t3422 = t3400 * t3377;
    (t3408, t3410, t3411, t3413, t3415, t3417, t3419, t3421, t3422)
}
