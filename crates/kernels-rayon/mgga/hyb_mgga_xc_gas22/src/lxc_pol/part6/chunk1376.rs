//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1376/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1376(t3470: f64, t238: f64, t242: f64, t25342: f64, t25345: f64, t25348: f64, t25359: f64, t25362: f64, t25365: f64, t29884: f64, t29888: f64, t29892: f64, t29896: f64, t29905: f64) -> (f64, f64) {
    let t29907 = t3470 * t3470;
    let t29909 = t238 * t242 * t29907;
    let t29911 = -0.33114e0_f64 * t29884 + 0.248355e0_f64 * t29888 + 0.49671e0_f64 * t29892 + 0.248355e0_f64 * t29896 - 0.33114e0_f64 * t25342 - 0.66228e0_f64 * t25345 - 0.33114e0_f64 * t25348 - 0.14717333333333333333e1_f64 * t25359 + 0.11038e1_f64 * t25362 + 0.11038e1_f64 * t25365 + 0.27595e0_f64 * t29905 + 0.49671e0_f64 * t29909;
    (t29909, t29911)
}
