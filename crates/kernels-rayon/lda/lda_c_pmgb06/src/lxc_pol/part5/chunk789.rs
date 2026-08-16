//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 789/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk789(t283: f64, t3672: f64, t3678: f64, t4518: f64, t4520: f64, t4522: f64, t4525: f64, t4527: f64, t6038: f64, t6071: f64, t6072: f64, t6074: f64, t7402: f64) -> f64 {
    let t7410 = -1.7544670867903938_f64 * t6038 - 24.0_f64 * t4518 + 36.0_f64 * t4520 + 60.0_f64 * t4522 + 0.0197516734986138_f64 * t7402 * t283 + 3.0_f64 * t6071 - 12.0_f64 * t6072 - 12.0_f64 * t6074 + 3.0_f64 * t4525 + 96.0_f64 * t4527 + t3672 - t3678;
    t7410
}
