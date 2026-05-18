//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 783/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk783<F: Float>(t12277: F, t609: F, t1580: F, t3337: F, t1969: F, t446: F, t1017: F, t1557: F, t1559: F, t9049: F, t11008: F, t9327: F) -> (F, F, F, F, F, F) {
    let t12278 = t12277 * t609;
    let t12283 = t3337 * t1580;
    let t12284 = t1969 * t12283;
    let t12285 = t446 * t12284;
    let t12287 = t1017 * t1557;
    let t12288 = t12287 * t1559;
    let t12289 = t9049 * t12288;
    let t12290 = t446 * t12289;
    let t12292 = t9327 * t11008;
    (t12278, t12283, t12285, t12288, t12290, t12292)
}
