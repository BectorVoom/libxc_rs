//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 797/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk797(t195: f64, t4369: f64, t421: f64, t423: f64, t4356: f64, t1478: f64, t295: f64, t1484: f64, t303: f64, t3972: f64, t306: f64, t1486: f64, t4363: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4371 = 1.0_f64 / t195 / t4369;
    let t4373 = t421 * t421;
    let t4374 = 1.0_f64 / t4373;
    let t4383 = t4356 * t423;
    let t4389 = t295 * t1478;
    let t4390 = 1.0_f64 / t1484;
    let t4394 = t303 * t3972;
    let t4397 = t306 * t3972;
    let t4400 = t4363 * t1486;
    (t4371, t4374, t4383, t4389, t4390, t4394, t4397, t4400)
}
