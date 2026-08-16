//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 565/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk565(t125: f64, t2414: f64, t1185: f64, t1189: f64, t1192: f64, t1197: f64, t1198: f64, t1199: f64, t81: f64) -> (f64, f64) {
    let t2415 = t125 * t2414;
    let t2422 = t1185 + t1189 - t1192 + t1197 + t81 + t1198 + t1199;
    (t2415, t2422)
}
