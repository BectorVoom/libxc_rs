//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1117/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1117<F: Float>(t2531: F, t2537: F, t2810: F, t10462: F, t975: F, t10461: F, t278: F, t299: F, t10497: F, t1138: F, t10496: F, t364: F, t392: F) -> (F, F, F, F, F, F) {
    let t31274 = t2531 * t2537;
    let t31296 = t2810 * t2810;
    let t31297 = F::new(1.0) / t31296;
    let t32896 = t975 * t10462;
    let t33822 = t278 / t10461 / t299;
    let t33853 = t1138 * t10497;
    let t33862 = t364 / t10496 / t392;
    (t31274, t31297, t32896, t33822, t33853, t33862)
}
