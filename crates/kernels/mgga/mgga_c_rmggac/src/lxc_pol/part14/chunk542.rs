//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 542/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk542<F: Float>(t739: F, t7530: F, t2024: F, t4905: F, t884: F, t2131: F, t942: F, t2124: F, t321: F, t446: F, t457: F, t201: F, t1979: F, t1982: F, t1162: F, t194: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7531 = t739 * t7530;
    let t7532 = 0.23948483403727617128e0 * t7531;
    let t7533 = t2024 * t4905;
    let t7534 = t884 * t7533;
    let t7535 = 0.23948483403727617128e0 * t7534;
    let t7536 = t942 * t2131;
    let t7538 = t2124 * t321;
    let t7539 = t739 * t7538;
    let t7540 = 0.11974241701863808564e0 * t7539;
    let t7541 = t446 * t457;
    let t7542 = t7541 * t201;
    let t7544 = t7542 * t1979 * t1982;
    let t7545 = 0.85129199786595678796e-5 * t7544;
    let t7546 = t194 * t1162;
    (t7532, t7533, t7535, t7536, t7538, t7540, t7541, t7542, t7545, t7546)
}
