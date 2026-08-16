//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 860/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk860(t5: f64, t2395: f64, t273: f64, t698: f64, t2377: f64, t3912: f64, t1068: f64, t2381: f64, t1072: f64, t2125: f64, t332: f64, t5961: f64, t9: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t6 = t5 <= zeta_threshold;
    let t6037 = t2395 * t273;
    let t6038 = t6037 * t698;
    let t6042 = t3912 * t2377;
    let t6047 = t1068 * t2381;
    let t6053 = piecewise3(t6, 0.0_f64, -8.0_f64 / 27.0_f64 * t6042 * t332 + 16.0_f64 / 9.0_f64 * t2125 * t1072 + 4.0_f64 / 9.0_f64 * t6047 * t332 + 4.0_f64 / 3.0_f64 * t9 * t5961);
    (t6037, t6038, t6042, t6047, t6053)
}
