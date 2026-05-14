//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1130/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1130<F: Float>(t1003: F, t13376: F, t26686: F, t14570: F, t283: F, t990: F, t1008: F, t2811: F, t4972: F, t27778: F, t3045: F, t1020: F, t26675: F, t27836: F, t9588: F, t1092: F, t1800: F, t3228: F) -> (F, F, F, F, F, F) {
    let t95636 = t26686 * t13376 * t1003;
    let t95640 = t14570 * t283 * t990;
    let t95645 = t26686 * t2811 * t4972 * t1008;
    let t95649 = t26686 * t27778 * t3045;
    let t95653 = t1020 * t27836 * t26675;
    let t95655 = t9588 * t283;
    let t95658 = t1092 * t95655 * t1800 * t3228;
    (t95636, t95640, t95645, t95649, t95653, t95658)
}
