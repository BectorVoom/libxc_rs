//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 646/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk646<F: Float>(t1220: F, t3569: F, t1210: F, t396: F, t404: F, t3551: F, t956: F, t962: F, t265: F, t3005: F, t3006: F, t971: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3570 = t3569 * t1220;
    let t3573 = t1210 * t1210;
    let t3574 = F::new(1.0) / t3573;
    let t3575 = t396 * t3574;
    let t3576 = t404 * t404;
    let t3577 = F::new(1.0) / t3576;
    let t3578 = t3551 * t3577;
    let t3582 = t956 * t962;
    let t3585 = t265 * t3005;
    let t3586 = t3006 * t971;
    (t3570, t3573, t3574, t3575, t3576, t3577, t3578, t3582, t3585, t3586)
}
