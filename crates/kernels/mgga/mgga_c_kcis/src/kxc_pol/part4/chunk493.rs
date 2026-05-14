//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 493/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk493<F: Float>(t2429: F, t776: F, t113: F, t717: F, t96: F, t89: F, t728: F) -> (F, F, F, F, F, F) {
    let t2430 = t2429 * t776;
    let t2434 = t113 * t717;
    let t2437 = t96 * t96;
    let t2438 = 1.0 / t2437;
    let t2439 = t89 * t2438;
    let t2440 = t728 * t728;
    (t2430, t2434, t2437, t2438, t2439, t2440)
}
