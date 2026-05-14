//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1119/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1119<F: Float>(t28383: F, t3728: F, t28357: F, t4142: F, t28510: F, t3717: F, t52460: F, t12234: F, t16836: F, t1943: F, t531: F, t11814: F, t28516: F, t2242: F, t4134: F, t1386: F, t16968: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t98162 = t3728 * t28383;
    let t98193 = t4142 * t28357;
    let t98225 = t4142 * t28510;
    let t98226 = 0.14739506172839506172e-2 * t98225;
    let t98233 = t52460 * t3717;
    let t98239 = t16836 * t12234;
    let t98240 = t1943 * t531;
    let t98254 = t11814 * t28516;
    let t98255 = 0.3684876543209876543e-2 * t98254;
    let t98266 = t2242 * t4134;
    let t98270 = t16968 * t1386;
    (t98162, t98193, t98225, t98226, t98233, t98239, t98240, t98254, t98255, t98266, t98270)
}
