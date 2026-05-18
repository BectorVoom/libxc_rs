//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 867/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk867<F: Float>(t127: F, t368: F, t3751: F, t1477: F, t3754: F, t1319: F, t5654: F, t1482: F, t1419: F, t1650: F, t11634: F, t1444: F) -> (F, F, F, F, F, F, F) {
    let t16353 = t127 * t368 * t3751;
    let t16354 = t1477 * t3754;
    let t16355 = t5654 * t1319;
    let t16356 = t16354 * t16355;
    let t16359 = t1482 * t3754;
    let t16360 = t5654 * t1419;
    let t16361 = t16359 * t16360;
    let t16364 = t1650 * t1319;
    let t16366 = t11634 * t16364 * t1419;
    let t16369 = t1477 * t1444;
    (t16353, t16355, t16356, t16360, t16361, t16366, t16369)
}
