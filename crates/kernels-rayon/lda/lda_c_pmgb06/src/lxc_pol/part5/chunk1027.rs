//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1027/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1027(t2002: f64, t6124: f64, t432: f64, t7503: f64, t1447: f64, t7535: f64, t2497: f64, t5194: f64, t16513: f64, t1893: f64, t439: f64, t1972: f64, t6533: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19280 = t2002 * t6124 / 15.0_f64;
    let t19282 = t432 * t7503 / 30.0_f64;
    let t19283 = t1447 * t7535;
    let t19284 = 4.0_f64 / 45.0_f64 * t19283;
    let t19285 = t5194 * t2497;
    let t19286 = 4.0_f64 / 45.0_f64 * t19285;
    let t19289 = t439 * t16513 * t1893 / 15.0_f64;
    let t19291 = 2.0_f64 / 15.0_f64 * t1972 * t6533;
    (t19280, t19282, t19284, t19286, t19289, t19291)
}
