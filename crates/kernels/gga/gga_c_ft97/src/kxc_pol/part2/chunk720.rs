//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 720/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk720<F: Float>(t11351: F, t35: F, t3064: F, t1711: F, t938: F, t371: F, t122: F, t409: F, t1751: F, t374: F, t930: F, t3021: F, t401: F) -> (F, F, F, F, F, F) {
    let t11352 = t11351 * t35;
    let t11353 = t3064 * t11352;
    let t11356 = t1711 * t938;
    let t11357 = t371 * t11356;
    let t11360 = t409 * t122;
    let t11361 = t371 * t11360;
    let t11368 = t374 * t930 * t1751;
    let t11371 = t3021 * t401;
    (t11353, t11356, t11357, t11361, t11368, t11371)
}
