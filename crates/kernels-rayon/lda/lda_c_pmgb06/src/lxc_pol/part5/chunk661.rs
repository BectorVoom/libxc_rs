//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 661/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk661(t5: f64, t1072: f64, t2192: f64, t330: f64, t332: f64, t5953: f64, t5958: f64, t5961: f64, t2386: f64, t3548: f64, t1219: f64, t2389: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t6 = t5 <= zeta_threshold;
    let t5965 = piecewise3(t6, 0.0_f64, 8.0_f64 / 27.0_f64 * t5953 * t332 - 8.0_f64 / 9.0_f64 * t2192 * t1072 - 2.0_f64 / 9.0_f64 * t5958 * t332 + 2.0_f64 / 3.0_f64 * t330 * t5961);
    let t5966 = t3548 * t2386;
    let t5971 = t1219 * t2389;
    let t5974 = -t5961;
    (t5965, t5966, t5971, t5974)
}
