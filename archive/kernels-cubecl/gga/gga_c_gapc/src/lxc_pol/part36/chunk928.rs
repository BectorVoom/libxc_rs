//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 928/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk928<F: Float>(t11329: F, t3714: F, t128: F, t1463: F, t1671: F, t1643: F, t3157: F, t3674: F, t561: F, t3085: F, t3664: F, t1453: F, t1457: F) -> (F, F, F, F, F, F, F) {
    let t11330 = t11329 * t3714;
    let t11332 = t1463 * t128;
    let t11333 = t1671 * t11332;
    let t11334 = t1643 * t11333;
    let t11337 = t561 * t3674 * t3157;
    let t11339 = t3664 * t3085;
    let t11341 = t1457 * t1453;
    (t11330, t11332, t11333, t11334, t11337, t11339, t11341)
}
