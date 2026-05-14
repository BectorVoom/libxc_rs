//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1113/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1113<F: Float>(t1020: F, t27836: F, t4806: F, t4548: F, t19553: F, t7718: F, t11068: F, t29111: F, t7788: F, t29107: F, t3500: F, t29159: F, t922: F, t92693: F, t1092: F, t1121: F, t6689: F, t95655: F) -> (F, F, F, F, F, F, F) {
    let t100420 = t1020 * t27836 * t4806;
    let t100423 = t1020 * t27836 * t4548;
    let t100426 = t1020 * t7718 * t19553;
    let t100429 = t7788 * t11068 * t29111;
    let t100432 = t7788 * t3500 * t29107;
    let t100436 = t92693 * t29159 * t922;
    let t100447 = t1092 * t95655 * t6689 * t1121;
    (t100420, t100423, t100426, t100429, t100432, t100436, t100447)
}
