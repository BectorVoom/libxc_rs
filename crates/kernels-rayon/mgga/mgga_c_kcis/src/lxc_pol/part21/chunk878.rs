//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 878/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk878(t1056: f64, t13480: f64, t10021: f64, t10026: f64, t10028: f64, t10033: f64, t10035: f64, t10037: f64, t10045: f64, t10048: f64, t10056: f64, t10058: f64, t111: f64, t120: f64, t13463: f64, t13468: f64, t13472: f64, t13473: f64, t13476: f64, t3158: f64, t4875: f64) -> f64 {
    let t13481 = t1056 * t13480;
    let t13484 = -0.10359077815592613752e-3_f64 * t4875 + 0.26416666666666666666e-2_f64 * t10021 + 0.23526125e-4_f64 * t10026 - 0.9368e-2_f64 * t10028 + 0.78420416666666666666e-4_f64 * t10033 + 0.4684e-2_f64 * t10035 - 0.15613333333333333333e-2_f64 * t10037 - 0.13208333333333333333e-2_f64 * t10045 + 0.88055555555555555553e-3_f64 * t10048 - 0.117630625e-4_f64 * t10056 + 0.15684083333333333333e-4_f64 * t10058 + 0.23911438650126355246e-1_f64 * t3158 - 0.52833333333333333333e-3_f64 * t111 * t13463 - 0.17611111111111111111e-3_f64 * t111 * t13468 + t13472 + 0.31368166666666666666e-4_f64 * t13473 - 0.10082625e-4_f64 * t120 * t13476 + 0.403305e-4_f64 * t120 * t13481;
    t13484
}
