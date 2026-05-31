//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1376/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1376<F: Float>(t3470: F, t238: F, t242: F, t25342: F, t25345: F, t25348: F, t25359: F, t25362: F, t25365: F, t29884: F, t29888: F, t29892: F, t29896: F, t29905: F) -> (F, F) {
    let t29907 = t3470 * t3470;
    let t29909 = t238 * t242 * t29907;
    let t29911 = -F::cast_from(0.33114e0_f64) * t29884 + F::cast_from(0.248355e0_f64) * t29888 + F::cast_from(0.49671e0_f64) * t29892 + F::cast_from(0.248355e0_f64) * t29896 - F::cast_from(0.33114e0_f64) * t25342 - F::cast_from(0.66228e0_f64) * t25345 - F::cast_from(0.33114e0_f64) * t25348 - F::cast_from(0.14717333333333333333e1_f64) * t25359 + F::cast_from(0.11038e1_f64) * t25362 + F::cast_from(0.11038e1_f64) * t25365 + F::cast_from(0.27595e0_f64) * t29905 + F::cast_from(0.49671e0_f64) * t29909;
    (t29909, t29911)
}
