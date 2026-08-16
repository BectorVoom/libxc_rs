//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 925/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk925(t2022: f64, t3: f64, t3177: f64, t675: f64, t2002: f64, t3178: f64, t39: f64, t698: f64, t701: f64, t3023: f64, t35: f64, t572: f64, t6007: f64, t6278: f64, t6279: f64, t6281: f64, t6283: f64, t6285: f64, t8288: f64, t8291: f64, t8293: f64, t8294: f64, t8299: f64, t8304: f64, t8309: f64, t8313: f64, t8317: f64) -> (f64, f64, f64, f64, f64) {
    let t8320 = t2022 * t3;
    let t8322 = t3177 * t8320 * t675;
    let t8326 = t3177 * t3178 * t2002;
    let t8329 = t698 * t39;
    let t8330 = t8329 * t701;
    let t8334 = -t6278 - 4.0_f64 / 243.0_f64 * t6279 + t6281 / 243.0_f64 - t6283 / 81.0_f64 + t6285 / 162.0_f64 - 2.0_f64 / 243.0_f64 * t8288 + t8291 - t8293 - 11.0_f64 / 81.0_f64 * t8294 - 5.0_f64 / 243.0_f64 * t572 * t8299 + 2.0_f64 / 27.0_f64 * t572 * t8304 + 4.0_f64 / 81.0_f64 * t3023 * t8309 - t572 * t8313 / 81.0_f64 - t572 * t8317 / 9.0_f64 - 4.0_f64 / 27.0_f64 * t3023 * t8322 + t572 * t8326 / 27.0_f64 + t35 * t6007 * t8330 / 27.0_f64;
    (t8322, t8326, t8329, t8330, t8334)
}
