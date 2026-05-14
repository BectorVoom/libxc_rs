//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 780/1135 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk780<F: Float>(t10293: F, t6951: F, t10328: F, t3239: F, t6935: F, t3258: F, t2206: F, t761: F, t2920: F, t3227: F, t297: F, t493: F, t7371: F, t3217: F, t3224: F, t8350: F) -> (F, F, F, F, F, F, F) {
    let t10329 = t10293 * t6951;
    let t10330 = t10328 * t10329;
    let t10332 = t3239 * t6935;
    let t10333 = t3258 * t10332;
    let t10335 = t761 * t2206;
    let t10336 = t2920 * t10335;
    let t10337 = t10336 * t3227;
    let t10339 = t493 * t297;
    let t10340 = t10339 * t7371;
    let t10341 = t3217 * t10340;
    let t10343 = t8350 * t3224;
    (t10330, t10333, t10335, t10336, t10337, t10341, t10343)
}
