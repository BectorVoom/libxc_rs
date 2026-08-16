//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 570/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk570<F: Float>(t7522: F, t2060: F, t4905: F, t903: F, t665: F, t798: F, t2024: F, t4048: F, t739: F, t884: F, t2131: F, t942: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7523 = F::cast_from(0.5987120850931904282e-1_f64) * t7522;
    let t7524 = t2060 * t4905;
    let t7525 = t903 * t7524;
    let t7526 = F::cast_from(0.8980681276397856423e-1_f64) * t7525;
    let t7527 = t665 * t798;
    let t7528 = t903 * t7527;
    let t7529 = F::cast_from(0.35922725105591425692e0_f64) * t7528;
    let t7530 = t2024 * t4048;
    let t7531 = t739 * t7530;
    let t7532 = F::cast_from(0.23948483403727617128e0_f64) * t7531;
    let t7533 = t2024 * t4905;
    let t7534 = t884 * t7533;
    let t7535 = F::cast_from(0.23948483403727617128e0_f64) * t7534;
    let t7536 = t942 * t2131;
    (t7523, t7524, t7526, t7527, t7529, t7530, t7532, t7533, t7535, t7536)
}
