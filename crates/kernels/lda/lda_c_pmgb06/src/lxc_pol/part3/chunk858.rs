//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 858/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk858<F: Float>(t11261: F, t11284: F, t38: F, t56: F, t8232: F, t8279: F, t370: F, t63: F, t8220: F, t8229: F, t8263: F, t8266: F, t8282: F, t8285: F, t8287: F, t8291: F) -> (F, F, F, F, F) {
    let t11286 = t11261 / 2.0 + t11284 / 2.0;
    let t11289 = 2.923025 * t38 * t56 * t11286;
    let t11296 = 2.923025 * t8232;
    let t11297 = 5.84605 * t8279;
    let t11299 = -1.46904 * t63 * t370 * t11286 - 3.0 / 2.0 * t8220 - 8.81424 * t8229 - t11296 + t8263 - t8266 + t11297 - 2.93808 * t8282 + t8285 + t8287 + t8291;
    (t11286, t11289, t11296, t11297, t11299)
}
