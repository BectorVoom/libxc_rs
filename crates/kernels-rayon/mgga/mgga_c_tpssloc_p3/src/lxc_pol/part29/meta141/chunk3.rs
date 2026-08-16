//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 791/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk791(t3120: f64, t360: f64, t1021: f64, t248: f64, t1013: f64, t361: f64, t363: f64, t3037: f64, t3033: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3121 = t3120 * t360;
    let t3123 = t248 * t1021 * t3121;
    let t3127 = 1.0_f64 / t1013 / t361;
    let t3128 = t3127 * t363;
    let t3129 = t3128 * t3037;
    let t3130 = t3033 * t3129;
    (t3121, t3123, t3127, t3128, t3129, t3130)
}
