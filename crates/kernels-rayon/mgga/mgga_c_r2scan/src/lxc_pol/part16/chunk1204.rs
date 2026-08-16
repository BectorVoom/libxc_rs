//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1204/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1204(t37947: f64, t37951: f64, t41600: f64, t43348: f64, t43351: f64, t43356: f64, t43359: f64, t43362: f64, t43365: f64, t43368: f64, t43372: f64, t22790: f64, t30057: f64, t3332: f64) -> (f64, f64) {
    let t43374 = 0.52396431978519890152e-1_f64 * t43348 + 0.10401866088065122276e1_f64 * t43351 + 0.15573871527278325618e-1_f64 * t37947 + 0.46721614581834976854e-1_f64 * t37951 - 0.47609969197673950971e-2_f64 * t43356 + 0.13099107994629972538e-1_f64 * t43359 - 0.28565981518604370583e-1_f64 * t43362 + 0.71414953796510926457e-2_f64 * t43365 - 0.69345773920434148507e0_f64 * t43368 + 0.46574606203128791245e-1_f64 * t43372 + t41600;
    let t43376 = t22790 * t3332 * t30057;
    (t43374, t43376)
}
