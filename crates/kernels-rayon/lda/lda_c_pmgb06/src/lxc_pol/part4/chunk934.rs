//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 934/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk934(t1795: f64, t868: f64, t2422: f64, t391: f64, t2414: f64, t81: f64, t199: f64, t1799: f64, t1808: f64, t2454: f64, t399: f64, t4187: f64, t4212: f64, t4214: f64, t5542: f64, t5551: f64, t5553: f64, t566: f64, t6928: f64, t6939: f64, t795: f64, t84: f64) -> (f64, f64) {
    let t6942 = t1795 * t868;
    let t6944 = t391 * t2422;
    let t6946 = t81 * t2414;
    let t6947 = t6946 * t199;
    let t6951 = t5542 + t5551 + t5553 - 0.0837628205355044_f64 * t6928 * t199 - 0.0837628205355044_f64 * t2454 * t566 - 0.1675256410710088_f64 * t1799 * t868 - 0.1675256410710088_f64 * t795 * t1808 - 0.0837628205355044_f64 * t399 * t2422 - 0.0837628205355044_f64 * t84 * t6939 + 0.1675256410710088_f64 * t6942 + 0.0837628205355044_f64 * t6944 + 0.0837628205355044_f64 * t6947 + t4187 - 0.1675256410710088_f64 * t4212 - 0.1675256410710088_f64 * t4214;
    (t6946, t6951)
}
