//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 944/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk944(t12: f64, t1080: f64, t395: f64, t247: f64, t337: f64, t1083: f64, t2136: f64, t642: f64, t1: f64, t1079: f64, t14: f64, t2133: f64, t2912: f64, t2938: f64, t3139: f64, t3922: f64, t4500: f64, t4503: f64, t764: f64, t8499: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t13 = t12 <= zeta_threshold;
    let t11039 = t395 * t1080;
    let t11047 = t247 * t337;
    let t11050 = t395 * t1083;
    let t11058 = 32.0_f64 * t2136 * t642;
    let t11060 = piecewise3(t13, 0.0_f64, 40.0_f64 / 81.0_f64 * t8499 * t764 * t2912 + 16.0_f64 / 9.0_f64 * t3922 * t1 * t11039 - 8.0_f64 / 9.0_f64 * t4500 * t3139 - 8.0_f64 / 3.0_f64 * t1079 * t395 * t337 + 8.0_f64 * t4503 * t11047 - 8.0_f64 / 3.0_f64 * t4503 * t11050 + 4.0_f64 / 9.0_f64 * t2133 * t2938 + 16.0_f64 * t14 * t247 - t11058);
    (t11039, t11047, t11050, t11060)
}
