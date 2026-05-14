//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 807/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk807<F: Float>(t2645: F, t5488: F, t3766: F, t1961: F, t3754: F, t2642: F, t11374: F, t1409: F, t3786: F, t1319: F, t1419: F, t16073: F, t5439: F, t1444: F, t1996: F, t3251: F) -> (F, F, F, F, F, F) {
    let t16378 = t5488 * t2645;
    let t16379 = t3766 * t16378;
    let t16382 = t1961 * t3754;
    let t16383 = t16382 * t2642;
    let t16384 = t11374 * t16383;
    let t16387 = t3786 * t1409;
    let t16388 = t1961 * t1319;
    let t16389 = t16388 * t1419;
    let t16390 = t16387 * t16389;
    let t16393 = t5439 * t16073;
    let t16397 = t1961 * t1444 * t2642;
    let t16398 = t3766 * t16397;
    let t16401 = t3251 * t1996;
    (t16379, t16384, t16390, t16393, t16398, t16401)
}
