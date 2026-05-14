//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1106/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1106<F: Float>(t12234: F, t16836: F, t1943: F, t531: F, t27357: F, t16823: F, t27370: F, t94229: F, t1394: F, t16810: F, t7923: F, t11814: F, t28516: F, t27364: F, t5649: F, t17270: F) -> (F, F, F, F, F, F, F) {
    let t98239 = t16836 * t12234;
    let t98240 = t1943 * t531;
    let t98242 = t98239 * t98240 * t27357;
    let t98246 = t27370 * t16823 * t94229;
    let t98252 = t1394 * t7923 * t16810;
    let t98254 = t11814 * t28516;
    let t98255 = 0.3684876543209876543e-2 * t98254;
    let t98257 = t1394 * t27364 * t5649;
    let t98260 = t1394 * t7923 * t17270;
    (t98242, t98246, t98252, t98254, t98255, t98257, t98260)
}
