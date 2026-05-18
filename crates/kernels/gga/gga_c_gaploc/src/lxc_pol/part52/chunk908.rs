//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 908/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk908<F: Float>(t11362: F, t9333: F, t1457: F, t1572: F, t44480: F, t44470: F, t13421: F, t1641: F, t13363: F, t37777: F, t11359: F, t40342: F) -> (F, F, F, F, F, F) {
    let t46291 = F::new(0.10725146985555128001e1) * t11362 * t9333;
    let t46294 = F::new(0.71500979903700853338e0) * t1572 * t1457 * t44480;
    let t46297 = F::new(0.71500979903700853338e0) * t1572 * t1457 * t44470;
    let t46299 = F::new(0.92023022289409799224e1) * t1641 * t13421;
    let t46301 = F::new(0.42900587942220512003e1) * t37777 * t13363;
    let t46303 = F::new(0.42900587942220512003e1) * t11359 * t40342;
    (t46291, t46294, t46297, t46299, t46301, t46303)
}
