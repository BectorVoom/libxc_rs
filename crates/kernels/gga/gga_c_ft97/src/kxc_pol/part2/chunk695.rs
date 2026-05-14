//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 695/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk695<F: Float>(t12283: F, t1969: F, t446: F, t1017: F, t1557: F, t1559: F, t9049: F, t11008: F, t9327: F, t11013: F, t2205: F, t3281: F, t2075: F, t925: F, t3052: F, t558: F) -> (F, F, F, F, F, F, F, F) {
    let t12284 = t1969 * t12283;
    let t12285 = t446 * t12284;
    let t12287 = t1017 * t1557;
    let t12288 = t12287 * t1559;
    let t12289 = t9049 * t12288;
    let t12290 = t446 * t12289;
    let t12292 = t9327 * t11008;
    let t12293 = t446 * t12292;
    let t12295 = t2205 * t11013;
    let t12296 = t3281 * t12295;
    let t12298 = t925 * t2075;
    let t12299 = t1969 * t12298;
    let t12300 = t446 * t12299;
    let t12302 = t3052 * t558;
    (t12285, t12288, t12290, t12293, t12296, t12298, t12300, t12302)
}
