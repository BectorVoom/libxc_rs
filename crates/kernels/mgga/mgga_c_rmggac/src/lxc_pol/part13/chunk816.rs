//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 816/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk816<F: Float>(t16503: F, t34962: F, t552: F, t7461: F, t22971: F, t7467: F, t1965: F, t1967: F, t28: F, t8511: F, t7478: F, t14243: F, t7482: F, t14237: F, t559: F, t2298: F, t26490: F) -> (F, F, F, F, F, F, F) {
    let t40270 = t16503 * t34962 * t552 * t7461;
    let t40274 = t16503 * t22971 * t552 * t7467;
    let t40278 = t8511 * t1965 * t1967 * t28;
    let t40279 = t40278 * t7478;
    let t40283 = t16503 * t14243 * t552 * t7482;
    let t40287 = t16503 * t14237 * t559 * t7461;
    let t40291 = t16503 * t14243 * t559 * t7467;
    let t40295 = t26490 * t2298;
    (t40270, t40274, t40279, t40283, t40287, t40291, t40295)
}
