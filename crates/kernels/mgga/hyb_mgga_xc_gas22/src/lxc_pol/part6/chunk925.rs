//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 925/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk925<F: Float>(t2022: F, t3: F, t3177: F, t675: F, t2002: F, t3178: F, t39: F, t698: F, t701: F, t3023: F, t35: F, t572: F, t6007: F, t6278: F, t6279: F, t6281: F, t6283: F, t6285: F, t8288: F, t8291: F, t8293: F, t8294: F, t8299: F, t8304: F, t8309: F, t8313: F, t8317: F) -> (F, F, F, F, F) {
    let t8320 = t2022 * t3;
    let t8322 = t3177 * t8320 * t675;
    let t8326 = t3177 * t3178 * t2002;
    let t8329 = t698 * t39;
    let t8330 = t8329 * t701;
    let t8334 = -t6278 - F::new(4.0) / F::new(243.0) * t6279 + t6281 / F::new(243.0) - t6283 / F::new(81.0) + t6285 / F::new(162.0) - F::new(2.0) / F::new(243.0) * t8288 + t8291 - t8293 - F::new(11.0) / F::new(81.0) * t8294 - F::new(5.0) / F::new(243.0) * t572 * t8299 + F::new(2.0) / F::new(27.0) * t572 * t8304 + F::new(4.0) / F::new(81.0) * t3023 * t8309 - t572 * t8313 / F::new(81.0) - t572 * t8317 / F::new(9.0) - F::new(4.0) / F::new(27.0) * t3023 * t8322 + t572 * t8326 / F::new(27.0) + t35 * t6007 * t8330 / F::new(27.0);
    (t8322, t8326, t8329, t8330, t8334)
}
