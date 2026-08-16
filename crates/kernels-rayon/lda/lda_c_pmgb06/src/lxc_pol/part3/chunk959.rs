//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 959/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk959(t11261: f64, t11284: f64, t38: f64, t56: f64, t8232: f64, t8279: f64, t370: f64, t63: f64, t8220: f64, t8229: f64, t8263: f64, t8266: f64, t8282: f64, t8285: f64, t8287: f64, t8291: f64) -> (f64, f64, f64, f64, f64) {
    let t11286 = t11261 / 2.0_f64 + t11284 / 2.0_f64;
    let t11289 = 2.923025_f64 * t38 * t56 * t11286;
    let t11296 = 2.923025_f64 * t8232;
    let t11297 = 5.84605_f64 * t8279;
    let t11299 = -1.46904_f64 * t63 * t370 * t11286 - 3.0_f64 / 2.0_f64 * t8220 - 8.81424_f64 * t8229 - t11296 + t8263 - t8266 + t11297 - 2.93808_f64 * t8282 + t8285 + t8287 + t8291;
    (t11286, t11289, t11296, t11297, t11299)
}
