//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1057/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1057<F: Float>(t31295: F, t487: F, t1487: F, t30205: F, t382: F, t486: F, t2275: F, t27204: F, t1471: F, t2059: F, t27331: F, t30153: F, t4272: F) -> (F, F, F, F, F) {
    let t31296 = t487 * t31295;
    let t31297 = t1487 * t31296;
    let t31299 = t382 * t30205;
    let t31300 = t487 * t31299;
    let t31301 = t486 * t31300;
    let t31303 = t27204 * t2275;
    let t31324 = t1471 * t27331 * t2059;
    let t31328 = t1471 * t4272 * t30153;
    (t31297, t31301, t31303, t31324, t31328)
}
