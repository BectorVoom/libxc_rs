//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 991/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk991(t1211: f64, t3542: f64, t1207: f64, t3574: f64, t3573: f64, t401: f64, t396: f64, t9725: f64, t9728: f64, t3549: f64, t3005: f64, t956: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10888 = t3542 * t1211;
    let t10893 = t1207 * t3574;
    let t10897 = 1.0_f64 / t3573 / t401;
    let t10898 = t396 * t10897;
    let t10923 = 0.16068111111111111111e1_f64 * t9725;
    let t10924 = 0.46308888888888888888e0_f64 * t9728;
    let t10936 = t1207 * t3549;
    let t10945 = 0.53272592592592592592e-1_f64 * t9725;
    let t10960 = t956 * t3005;
    (t10888, t10893, t10898, t10923, t10924, t10936, t10945, t10960)
}
