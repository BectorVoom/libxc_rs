//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 983/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk983<F: Float>(t12830: F, t14496: F, t14497: F, t3904: F, t442: F, t1056: F, t1471: F, t13125: F, t196: F, t13212: F, t13216: F, t13227: F, t140: F, t14444: F, t14446: F, t14449: F, t14453: F, t14458: F, t14461: F, t14464: F, t14469: F, t14477: F, t14481: F, t14486: F, t14489: F, t14493: F, t1470: F, t4253: F, t460: F, t479: F, t6278: F) -> F {
    let t14499 = t14496 * t14497 * t12830;
    let t14502 = t3904 * t442;
    let t14504 = t1471 * t14502 * t1056;
    let t14507 = t13125 * t196;
    let t14510 = -F::cast_from(0.53062222222222222221e-1_f64) * t14444 - F::cast_from(0.88437037037037037035e-1_f64) * t14446 - F::cast_from(0.79593333333333333333e-1_f64) * t1470 * t14449 - F::cast_from(0.26531111111111111111e-1_f64) * t1470 * t14453 - F::cast_from(0.27860625e-1_f64) * t4253 * t13216 + F::cast_from(0.15918666666666666666e0_f64) * t6278 * t14458 - F::cast_from(0.13265555555555555555e0_f64) * t6278 * t14461 - F::cast_from(0.39796666666666666666e-1_f64) * t140 * t479 * t14464 + F::cast_from(0.26531111111111111111e-1_f64) * t14469 + F::cast_from(0.5572125e-1_f64) * t4253 * t13227 - F::cast_from(0.27860625e-1_f64) * t4253 * t13212 + F::cast_from(0.15918666666666666666e0_f64) * t1470 * t14477 - F::cast_from(0.15918666666666666667e0_f64) * t1470 * t14481 + F::cast_from(0.26531111111111111111e0_f64) * t1470 * t14486 + F::cast_from(0.10612444444444444444e0_f64) * t14489 - F::cast_from(0.13265555555555555556e0_f64) * t1470 * t14493 - F::cast_from(0.11791604938271604938e0_f64) * t1470 * t14499 - F::cast_from(0.79593333333333333333e-1_f64) * t1470 * t14504 + F::cast_from(0.619125e-2_f64) * t14507 * t460;
    t14510
}
