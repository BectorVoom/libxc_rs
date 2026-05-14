//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1058/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1058<F: Float>(t1390: F, t2209: F, t1471: F, t3278: F, t3283: F, t6298: F, t3532: F, t4271: F, t2242: F, t306: F, t140: F, t2253: F, t430: F, t20792: F, t416: F, t1429: F, t1434: F, t14469: F, t14489: F, t1460: F, t1470: F, t19363: F, t2225: F, t3560: F, t3566: F, t3588: F, t3594: F, t3620: F, t4244: F, t479: F, t5958: F, t6247: F, t6267: F) -> (F,) {
    let t21230 = t2209 * t1390;
    let t21232 = t1471 * t21230 * t3278;
    let t21236 = t1471 * t6298 * t3283;
    let t21239 = t2209 * t3532;
    let t21241 = t4271 * t21239 * t3278;
    let t21252 = t2242 * t306;
    let t21256 = t140 * t430 * t2253;
    let t21258 = t416 * t20792;
    let t21266 = -0.619125e-2 * t4244 * t2225 - 0.123825e-1 * t1460 * t5958 + 0.17687407407407407407e-1 * t14469 - 0.371475e-1 * t6267 * t19363 + 0.35374814814814814814e-1 * t14489 + 0.53062222222222222222e-1 * t1470 * t21232 - 0.26531111111111111111e-1 * t1470 * t21236 - 0.44218518518518518518e-1 * t1470 * t21241 + 0.1857375e-1 * t6247 * t1429 - 0.123825e-1 * t6247 * t1434 + 0.9286875e-2 * t2242 * t3588 - 0.619125e-2 * t2242 * t3620 - 0.1857375e-1 * t21252 * t3566 + 0.88437037037037037037e-2 * t21256 - 0.39796666666666666666e-1 * t140 * t479 * t21258 + 0.123825e-1 * t2242 * t3594 + 0.46434375e-2 * t2242 * t3560;
    (t21266,)
}
