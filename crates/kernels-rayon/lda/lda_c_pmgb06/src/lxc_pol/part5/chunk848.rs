//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 848/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk848(t1100: f64, t79: f64, t4320: f64, t711: f64, t715: f64, t20: f64, t369: f64, t3501: f64, t3502: f64, t642: f64, t3509: f64, t3510: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8193 = t79 * t1100;
    let t8194 = 120.0_f64 * t8193;
    let t8208 = t4320 * t711;
    let t8211 = 0.7805426614091894_f64 * t4320 * t715;
    let t8245 = 1.0_f64 / t369 / t20;
    let t8263 = 15.589466666666667_f64 * t3501 * t3502 * t642;
    let t8266 = 2.6116266666666665_f64 * t3509 * t3510 * t642;
    (t8193, t8194, t8208, t8211, t8245, t8263, t8266)
}
