//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 876/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk876<F: Float>(t1056: F, t14502: F, t1471: F, t13125: F, t196: F, t13212: F, t13216: F, t13227: F, t140: F, t14444: F, t14446: F, t14449: F, t14453: F, t14458: F, t14461: F, t14464: F, t14469: F, t14477: F, t14481: F, t14486: F, t14489: F, t14493: F, t14499: F, t1470: F, t4253: F, t460: F, t479: F, t6278: F) -> (F,) {
    let t14504 = t1471 * t14502 * t1056;
    let t14507 = t13125 * t196;
    let t14510 = -0.53062222222222222221e-1 * t14444 - 0.88437037037037037035e-1 * t14446 - 0.79593333333333333333e-1 * t1470 * t14449 - 0.26531111111111111111e-1 * t1470 * t14453 - 0.27860625e-1 * t4253 * t13216 + 0.15918666666666666666e0 * t6278 * t14458 - 0.13265555555555555555e0 * t6278 * t14461 - 0.39796666666666666666e-1 * t140 * t479 * t14464 + 0.26531111111111111111e-1 * t14469 + 0.5572125e-1 * t4253 * t13227 - 0.27860625e-1 * t4253 * t13212 + 0.15918666666666666666e0 * t1470 * t14477 - 0.15918666666666666667e0 * t1470 * t14481 + 0.26531111111111111111e0 * t1470 * t14486 + 0.10612444444444444444e0 * t14489 - 0.13265555555555555556e0 * t1470 * t14493 - 0.11791604938271604938e0 * t1470 * t14499 - 0.79593333333333333333e-1 * t1470 * t14504 + 0.619125e-2 * t14507 * t460;
    (t14510,)
}
