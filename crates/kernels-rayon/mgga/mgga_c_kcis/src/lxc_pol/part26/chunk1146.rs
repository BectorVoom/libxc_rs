//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1146/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1146(t27370: f64, t29288: f64, t27369: f64, t28336: f64, t28369: f64, t28392: f64, t28395: f64, t29259: f64, t29267: f64, t29271: f64, t29275: f64, t29278: f64, t29281: f64, t29284: f64, t7908: f64, t8144: f64, t8148: f64, t8155: f64) -> (f64, f64) {
    let t29289 = t27370 * t29288;
    let t29296 = -0.15445601851851851852e-3_f64 * t28336 + 0.46336805555555555556e-3_f64 * t7908 * t29259 - 0.46336805555555555556e-3_f64 * t28369 * t8155 + 0.12356481481481481482e-2_f64 * t28392 * t8155 + 0.33163888888888888888e-2_f64 * t29267 + 0.16581944444444444444e-2_f64 * t29271 + 0.27636574074074074073e-2_f64 * t29275 - 0.33163888888888888888e-2_f64 * t29278 + 0.22109259259259259258e-2_f64 * t29281 + 0.61836467013888888889e-4_f64 * t27369 * t29284 + 0.22109259259259259258e-2_f64 * t28395 - 0.13901041666666666667e-2_f64 * t7908 * t29289 - 0.18550940104166666667e-3_f64 * t27369 * t29289 + 0.13901041666666666667e-2_f64 * t8144 * t8148;
    (t29289, t29296)
}
