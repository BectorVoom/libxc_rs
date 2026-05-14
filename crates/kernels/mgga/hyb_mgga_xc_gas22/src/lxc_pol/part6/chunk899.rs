//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 899/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk899<F: Float>(t2054: F, t3: F, t3171: F, t675: F, t2002: F, t3172: F, t2028: F, t3177: F, t2022: F, t3178: F, t39: F, t698: F, t701: F, t3023: F, t35: F, t572: F, t6007: F, t6278: F, t6279: F, t6281: F, t6283: F, t6285: F, t8288: F, t8291: F, t8293: F, t8294: F, t8299: F, t8304: F) -> (F, F, F, F, F, F, F, F) {
    let t8307 = t2054 * t3;
    let t8309 = t3171 * t8307 * t675;
    let t8313 = t3171 * t3172 * t2002;
    let t8317 = t3177 * t3172 * t2028;
    let t8320 = t2022 * t3;
    let t8322 = t3177 * t8320 * t675;
    let t8326 = t3177 * t3178 * t2002;
    let t8329 = t698 * t39;
    let t8330 = t8329 * t701;
    let t8334 = -t6278 - 4.0 / 243.0 * t6279 + t6281 / 243.0 - t6283 / 81.0 + t6285 / 162.0 - 2.0 / 243.0 * t8288 + t8291 - t8293 - 11.0 / 81.0 * t8294 - 5.0 / 243.0 * t572 * t8299 + 2.0 / 27.0 * t572 * t8304 + 4.0 / 81.0 * t3023 * t8309 - t572 * t8313 / 81.0 - t572 * t8317 / 9.0 - 4.0 / 27.0 * t3023 * t8322 + t572 * t8326 / 27.0 + t35 * t6007 * t8330 / 27.0;
    (t8309, t8313, t8317, t8322, t8326, t8329, t8330, t8334)
}
