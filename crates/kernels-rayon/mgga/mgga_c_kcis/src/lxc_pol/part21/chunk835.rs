//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 835/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk835(t265: f64, t9825: f64, t9630: f64, t1211: f64, t3542: f64, t1207: f64, t3574: f64, t3573: f64, t401: f64, t396: f64, t9725: f64, t9728: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10877 = t265 * t9825;
    let t10884 = t265 * t9630;
    let t10888 = t3542 * t1211;
    let t10893 = t1207 * t3574;
    let t10897 = 1.0_f64 / t3573 / t401;
    let t10898 = t396 * t10897;
    let t10923 = 0.16068111111111111111e1_f64 * t9725;
    let t10924 = 0.46308888888888888888e0_f64 * t9728;
    (t10877, t10884, t10888, t10893, t10898, t10923, t10924)
}
