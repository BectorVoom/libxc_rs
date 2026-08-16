//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 783/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk783(t8563: f64, t8565: f64, t8569: f64, t8572: f64, t8578: f64, t8583: f64, t8585: f64, t8588: f64, t8590: f64, t8593: f64, t8595: f64, t8598: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t38254 = 0.27274661654245341728e-1_f64 * t8563;
    let t38255 = 0.68186654135613354322e-2_f64 * t8565;
    let t38256 = 0.68186654135613354322e-2_f64 * t8569;
    let t38257 = 0.85129199786595678796e-5_f64 * t8572;
    let t38260 = 0.85129199786595678796e-5_f64 * t8578;
    let t38261 = 0.85129199786595678796e-5_f64 * t8583;
    let t38262 = 0.25538759935978703638e-4_f64 * t8585;
    let t38263 = 0.25538759935978703638e-4_f64 * t8588;
    let t38266 = 0.25538759935978703638e-4_f64 * t8590;
    let t38267 = 0.25538759935978703638e-4_f64 * t8593;
    let t38268 = 0.85129199786595678796e-5_f64 * t8595;
    let t38269 = 0.85129199786595678796e-5_f64 * t8598;
    (t38254, t38255, t38256, t38257, t38260, t38261, t38262, t38263, t38266, t38267, t38268, t38269)
}
