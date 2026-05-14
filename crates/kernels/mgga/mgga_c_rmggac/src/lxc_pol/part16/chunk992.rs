//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 992/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk992<F: Float>(t118: F, t1652: F, t333: F, t44339: F, t44340: F, t44341: F, t44362: F, t4669: F, t46765: F, t46770: F, t46772: F, t46774: F, t46782: F, t46784: F, t48432: F, t49560: F, t5148: F, t9523: F) -> (F,) {
    let t49606 = -0.35922725105591425692e0 * t4669 * t49560 * t333 + t44339 - t44340 - t44341 - 0.23948483403727617128e0 * t5148 * t9523 * t1652 + 0.79828278012425390427e-1 * t46765 - 0.17961362552795712846e0 * t46770 + 0.35922725105591425692e0 * t46772 + 0.8980681276397856423e-1 * t46774 - t44362 - 0.39914139006212695214e-1 * t118 * t48432 - 0.17961362552795712846e0 * t46782 + 0.17961362552795712846e0 * t46784;
    (t49606,)
}
