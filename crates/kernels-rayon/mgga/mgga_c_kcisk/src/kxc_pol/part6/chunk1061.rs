//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1061/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1061(t140: f64, t1470: f64, t21252: f64, t21256: f64, t2242: f64, t27308: f64, t27319: f64, t27321: f64, t27355: f64, t31024: f64, t31041: f64, t31045: f64, t31081: f64, t31106: f64, t31114: f64, t31118: f64, t31352: f64, t31356: f64, t31379: f64, t31385: f64, t31388: f64, t4253: f64, t476: f64, t479: f64, t6256: f64, t6267: f64, t6278: f64, t7869: f64, t7878: f64) -> f64 {
    let t31392 = -0.13265555555555555556e0_f64 * t1470 * t31352 - 0.11791604938271604938e0_f64 * t1470 * t31356 - 0.53062222222222222221e-1_f64 * t27308 - 0.88437037037037037035e-1_f64 * t27319 - 0.10612444444444444444e0_f64 * t27321 + 0.5572125e-1_f64 * t4253 * t31081 - 0.139303125e-1_f64 * t6256 * t31114 + 0.139303125e-1_f64 * t6256 * t31118 - 0.79593333333333333333e-1_f64 * t27355 - 0.27860625e-1_f64 * t4253 * t31045 - 0.232171875e-2_f64 * t476 * t31024 + 0.371475e-1_f64 * t2242 * t7878 + 0.371475e-1_f64 * t6267 * t31106 - 0.27860625e-1_f64 * t4253 * t31041 - 0.39796666666666666666e-1_f64 * t140 * t479 * t31379 - 0.5572125e-1_f64 * t21252 * t7869 + 0.15918666666666666666e0_f64 * t6278 * t31385 - 0.13265555555555555555e0_f64 * t6278 * t31388 + 0.26531111111111111111e-1_f64 * t21256;
    t31392
}
