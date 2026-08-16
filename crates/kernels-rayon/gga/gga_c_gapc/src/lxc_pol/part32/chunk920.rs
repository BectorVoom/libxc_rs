//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 920/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk920(t11329: f64, t3714: f64, t128: f64, t1463: f64, t1671: f64, t1643: f64, t3157: f64, t3674: f64, t561: f64, t3085: f64, t3664: f64, t1453: f64, t1457: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11330 = t11329 * t3714;
    let t11332 = t1463 * t128;
    let t11333 = t1671 * t11332;
    let t11334 = t1643 * t11333;
    let t11337 = t561 * t3674 * t3157;
    let t11339 = t3664 * t3085;
    let t11341 = t1457 * t1453;
    (t11330, t11332, t11333, t11334, t11337, t11339, t11341)
}
