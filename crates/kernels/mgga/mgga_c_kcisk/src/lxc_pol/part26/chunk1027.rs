//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1027/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1027<F: Float>(t25469: F, t6279: F, t1471: F, t4277: F, t7710: F, t1472: F, t25312: F, t14434: F, t14469: F, t1470: F, t18081: F, t21252: F, t21256: F, t26609: F, t27345: F, t27348: F, t27351: F, t27355: F, t27358: F, t27362: F, t27365: F, t27371: F, t27374: F, t5933: F, t6267: F, t6278: F, t7869: F) -> (F,) {
    let t27377 = t6279 * t25469;
    let t27383 = t1471 * t4277 * t7710;
    let t27387 = t1471 * t1472 * t25312;
    let t27393 = 0.21224888888888888889e0 * t18081 * t27345 + 0.53062222222222222222e-1 * t6278 * t27348 - 0.44218518518518518518e-1 * t6278 * t27351 - 0.26531111111111111111e-1 * t27355 + 0.10612444444444444444e0 * t6278 * t27358 - 0.88437037037037037037e-1 * t6278 * t27362 - 0.15918666666666666667e0 * t6278 * t27365 + 0.88437037037037037037e-2 * t14469 - 0.1857375e-1 * t14434 * t7869 + 0.26531111111111111111e0 * t6278 * t27371 - 0.11791604938271604938e0 * t6278 * t27374 - 0.17687407407407407407e0 * t18081 * t27377 - 0.371475e-1 * t6267 * t26609 - 0.26531111111111111111e-1 * t1470 * t27383 - 0.26531111111111111111e-1 * t1470 * t27387 - 0.1857375e-1 * t21252 * t5933 + 0.17687407407407407407e-1 * t21256;
    (t27393,)
}
