//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 566/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk566(t5: f64, t12: f64, t113: f64, t2414: f64, t301: f64, t1212: f64, t2377: f64, t2381: f64, t330: f64, t1219: f64, t2386: f64, t2389: f64, t336: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6 = t5 <= zeta_threshold;
    let t13 = t12 <= zeta_threshold;
    let t2432 = t2414 * t113 * t301;
    let t2435 = t1212 * t2377;
    let t2437 = t330 * t2381;
    let t2440 = piecewise3(t6, 0.0_f64, -2.0_f64 / 9.0_f64 * t2435 + 2.0_f64 / 3.0_f64 * t2437);
    let t2441 = t1219 * t2386;
    let t2443 = t336 * t2389;
    let t2446 = piecewise3(t13, 0.0_f64, -2.0_f64 / 9.0_f64 * t2441 + 2.0_f64 / 3.0_f64 * t2443);
    let t2448 = t2440 / 2.0_f64 + t2446 / 2.0_f64;
    (t2432, t2435, t2437, t2441, t2443, t2448)
}
