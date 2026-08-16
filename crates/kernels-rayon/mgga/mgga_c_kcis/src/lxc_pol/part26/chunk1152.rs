//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1152/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1152(t29372: f64, t303: f64, t2237: f64, t27339: f64, t28465: f64, t28467: f64, t29324: f64, t29338: f64, t29341: f64, t29344: f64, t29355: f64, t29358: f64, t29362: f64, t29366: f64, t29370: f64, t7908: f64, t8148: f64, t8151: f64, t8159: f64) -> (f64, f64) {
    let t29373 = t303 * t29372;
    let t29377 = -0.49745833333333333332e-2_f64 * t29338 + 0.33163888888888888888e-2_f64 * t29341 + 0.69505208333333333333e-3_f64 * t2237 * t29344 - 0.37069444444444444444e-2_f64 * t8151 * t8159 - 0.37069444444444444444e-2_f64 * t8151 * t8148 - 0.185671721767578125e-4_f64 * t27339 * t29324 - 0.33163888888888888888e-2_f64 * t29355 - 0.23168402777777777778e-3_f64 * t7908 * t29358 - 0.30891203703703703704e-3_f64 * t7908 * t29362 - 0.88437037037037037034e-2_f64 * t29366 - 0.33163888888888888888e-2_f64 * t29370 + 0.24872916666666666666e-2_f64 * t29373 + 0.33163888888888888888e-2_f64 * t28465 - 0.46336805555555555556e-3_f64 * t28467;
    (t29373, t29377)
}
