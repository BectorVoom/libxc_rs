//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 983/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk983(t12830: f64, t14496: f64, t14497: f64, t3904: f64, t442: f64, t1056: f64, t1471: f64, t13125: f64, t196: f64, t13212: f64, t13216: f64, t13227: f64, t140: f64, t14444: f64, t14446: f64, t14449: f64, t14453: f64, t14458: f64, t14461: f64, t14464: f64, t14469: f64, t14477: f64, t14481: f64, t14486: f64, t14489: f64, t14493: f64, t1470: f64, t4253: f64, t460: f64, t479: f64, t6278: f64) -> f64 {
    let t14499 = t14496 * t14497 * t12830;
    let t14502 = t3904 * t442;
    let t14504 = t1471 * t14502 * t1056;
    let t14507 = t13125 * t196;
    let t14510 = -0.53062222222222222221e-1_f64 * t14444 - 0.88437037037037037035e-1_f64 * t14446 - 0.79593333333333333333e-1_f64 * t1470 * t14449 - 0.26531111111111111111e-1_f64 * t1470 * t14453 - 0.27860625e-1_f64 * t4253 * t13216 + 0.15918666666666666666e0_f64 * t6278 * t14458 - 0.13265555555555555555e0_f64 * t6278 * t14461 - 0.39796666666666666666e-1_f64 * t140 * t479 * t14464 + 0.26531111111111111111e-1_f64 * t14469 + 0.5572125e-1_f64 * t4253 * t13227 - 0.27860625e-1_f64 * t4253 * t13212 + 0.15918666666666666666e0_f64 * t1470 * t14477 - 0.15918666666666666667e0_f64 * t1470 * t14481 + 0.26531111111111111111e0_f64 * t1470 * t14486 + 0.10612444444444444444e0_f64 * t14489 - 0.13265555555555555556e0_f64 * t1470 * t14493 - 0.11791604938271604938e0_f64 * t1470 * t14499 - 0.79593333333333333333e-1_f64 * t1470 * t14504 + 0.619125e-2_f64 * t14507 * t460;
    t14510
}
