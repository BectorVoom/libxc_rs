//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1220/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1220(t9404: f64, t132: f64, t137: f64, t1395: f64, t6225: f64, t161: f64, t489: f64, t6448: f64, t1848: f64, t2095: f64, t4945: f64, t831: f64) -> (f64, f64, f64, f64, f64) {
    let t16083 = 2.0_f64 / 135.0_f64 * t9404;
    let t16087 = t132 * t137 * t1395 * t6225 / 15.0_f64;
    let t16089 = t161 * t489 * t6448;
    let t16090 = 4.0_f64 / 45.0_f64 * t16089;
    let t16092 = 2.0_f64 / 15.0_f64 * t1848 * t2095;
    let t16094 = t831 * t4945 / 15.0_f64;
    (t16083, t16087, t16090, t16092, t16094)
}
