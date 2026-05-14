//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 984/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk984<F: Float>(t442: F, t7764: F, t1056: F, t3539: F, t7877: F, t13138: F, t2192: F, t5703: F, t5953: F, t459: F, t1175: F, t5926: F, t425: F, t1364: F, t3564: F, t3521: F, t7858: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26553 = t7764 * t442;
    let t26554 = t26553 * t1056;
    let t26555 = t3539 * t26554;
    let t26558 = t7877 * t442;
    let t26559 = t26558 * t1056;
    let t26560 = t13138 * t26559;
    let t26563 = t2192 * t5703;
    let t26564 = t5953 * t26563;
    let t26567 = t459 * t7764;
    let t26568 = t26567 * t1175;
    let t26569 = t5926 * t26568;
    let t26572 = t425 * t7764;
    let t26573 = t26572 * t1364;
    let t26574 = t3564 * t26573;
    let t26577 = t3521 * t7858;
    (t26555, t26560, t26563, t26564, t26568, t26569, t26573, t26574, t26577)
}
