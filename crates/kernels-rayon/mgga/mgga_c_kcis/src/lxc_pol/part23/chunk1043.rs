//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1043/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1043(t2237: f64, t27377: f64, t27381: f64, t27385: f64, t27389: f64, t27392: f64, t27396: f64, t27400: f64, t27403: f64, t27410: f64, t7895: f64, t7901: f64, t7916: f64) -> f64 {
    let t27413 = -0.15445601851851851852e-3_f64 * t27377 + 0.16581944444444444444e-2_f64 * t27381 + 0.27636574074074074073e-2_f64 * t27385 - 0.33163888888888888888e-2_f64 * t27389 - 0.24872916666666666666e-2_f64 * t27392 - 0.13901041666666666667e-2_f64 * t2237 * t27396 + 0.16581944444444444444e-2_f64 * t27400 + 0.69505208333333333333e-3_f64 * t2237 * t27403 + 0.13901041666666666667e-2_f64 * t7895 * t7916 + 0.13901041666666666667e-2_f64 * t7895 * t7901 + 0.18550940104166666667e-3_f64 * t27410 * t7901;
    t27413
}
