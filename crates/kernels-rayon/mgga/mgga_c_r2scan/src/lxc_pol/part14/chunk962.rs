//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 962/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk962(t11199: f64, t3262: f64, t3264: f64, t3275: f64, t3352: f64, t11031: f64, t11057: f64, t11034: f64, t11037: f64, t11039: f64, t11041: f64, t11043: f64, t11045: f64, t11048: f64, t11051: f64, t11054: f64) -> (f64, f64, f64, f64, f64) {
    let t11201 = t3262 * t11199 * t3264;
    let t11202 = 3.0_f64 / 2.0_f64 * t11201;
    let t11204 = t3275 * t11199 * t3352;
    let t11205 = t11204 / 2.0_f64;
    let t11206 = 22.0_f64 / 9.0_f64 * t11031;
    let t11215 = 22.0_f64 / 9.0_f64 * t11057;
    let t11216 = -t11206 - 4.0_f64 / 3.0_f64 * t11034 - t11037 / 2.0_f64 + t11039 / 4.0_f64 - t11041 / 4.0_f64 + t11043 + 4.0_f64 / 3.0_f64 * t11045 - 3.0_f64 / 2.0_f64 * t11048 - 8.0_f64 / 3.0_f64 * t11051 + t11054 / 2.0_f64 - t11215;
    (t11202, t11205, t11206, t11215, t11216)
}
