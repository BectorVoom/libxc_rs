//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 959/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk959<F: Float>(t11261: F, t11284: F, t38: F, t56: F, t8232: F, t8279: F, t370: F, t63: F, t8220: F, t8229: F, t8263: F, t8266: F, t8282: F, t8285: F, t8287: F, t8291: F) -> (F, F, F, F, F) {
    let t11286 = t11261 / F::cast_from(2.0_f64) + t11284 / F::cast_from(2.0_f64);
    let t11289 = F::cast_from(2.923025_f64) * t38 * t56 * t11286;
    let t11296 = F::cast_from(2.923025_f64) * t8232;
    let t11297 = F::cast_from(5.84605_f64) * t8279;
    let t11299 = -F::cast_from(1.46904_f64) * t63 * t370 * t11286 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t8220 - F::cast_from(8.81424_f64) * t8229 - t11296 + t8263 - t8266 + t11297 - F::cast_from(2.93808_f64) * t8282 + t8285 + t8287 + t8291;
    (t11286, t11289, t11296, t11297, t11299)
}
