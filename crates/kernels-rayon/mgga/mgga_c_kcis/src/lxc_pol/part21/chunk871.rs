//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 871/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk871(t13410: f64, t13411: f64, t3200: f64, t13367: f64, t13370: f64, t13373: f64, t13377: f64, t13382: f64, t13389: f64, t13391: f64, t13394: f64, t13399: f64, t13403: f64, t13406: f64, t13409: f64, t2812: f64, t2836: f64, t3046: f64, t4981: f64, t9574: f64, t9576: f64, t9581: f64, t9600: f64, t979: f64) -> (f64, f64) {
    let t13412 = t13410 * t13411;
    let t13413 = t3200 * t13412;
    let t13415 = 0.33163888888888888888e-2_f64 * t9574 + 0.88437037037037037034e-2_f64 * t9576 - 0.16581944444444444444e-2_f64 * t9581 + 0.16581944444444444444e-2_f64 * t13367 - 0.24872916666666666666e-2_f64 * t13370 - 0.49745833333333333332e-2_f64 * t13373 - 0.22109259259259259258e-2_f64 * t9600 + 0.13345e0_f64 * t979 * t13377 + 0.178089025e-1_f64 * t2836 * t13377 - 0.58958024691358024689e-2_f64 * t13382 + 0.66725e-1_f64 * t4981 * t2812 - 0.66725e-1_f64 * t4981 * t3046 - 0.33163888888888888888e-2_f64 * t13389 + 0.22109259259259259258e-2_f64 * t13391 - 0.88437037037037037034e-2_f64 * t13394 - 0.55273148148148148147e-2_f64 * t13399 - 0.17687407407407407407e-1_f64 * t13403 - 0.88437037037037037034e-2_f64 * t13406 + t13409 - 0.44218518518518518517e-2_f64 * t13413;
    (t13413, t13415)
}
