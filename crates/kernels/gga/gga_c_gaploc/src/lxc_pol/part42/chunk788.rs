//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 788/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk788<F: Float>(t1457: F, t1572: F, t44470: F, t13421: F, t1641: F, t13363: F, t37777: F, t11359: F, t40342: F, t2492: F, t4752: F, t3377: F, t38181: F, t41884: F, t11549: F, t20535: F, t2478: F) -> (F, F, F, F, F, F, F, F) {
    let t46297 = 0.71500979903700853338e0 * t1572 * t1457 * t44470;
    let t46299 = 0.92023022289409799224e1 * t1641 * t13421;
    let t46301 = 0.42900587942220512003e1 * t37777 * t13363;
    let t46303 = 0.42900587942220512003e1 * t11359 * t40342;
    let t46311 = 0.28600391961480341335e1 * t11359 * t4752 * t2492;
    let t46316 = 0.10725146985555128001e1 * t38181 * t3377;
    let t46327 = 0.71500979903700853339e0 * t41884;
    let t46331 = t20535 * t11549 * t2478;
    (t46297, t46299, t46301, t46303, t46311, t46316, t46327, t46331)
}
