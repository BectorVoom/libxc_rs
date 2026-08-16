//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 500/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk500(t374: f64, t376: f64, t677: f64, t370: f64, t121: f64, t1013: f64, t361: f64, t363: f64, t3037: f64, t3033: f64, t360: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3082 = t374 * t677 * t376;
    let t3084 = t370 * t3082 / 13824.0_f64;
    let t3101 = t121 * t376;
    let t3127 = 1.0_f64 / t1013 / t361;
    let t3128 = t3127 * t363;
    let t3129 = t3128 * t3037;
    let t3130 = t3033 * t3129;
    let t3131 = t360 * t360;
    (t3082, t3084, t3101, t3127, t3128, t3129, t3130, t3131)
}
