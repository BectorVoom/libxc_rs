//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1335/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1335(t2002: f64, t5365: f64, t5350: f64, t4754: f64, t815: f64, t1874: f64, t1887: f64, t4682: f64, t802: f64, t17527: f64, t17530: f64, t17532: f64, t17534: f64, t17537: f64, t17542: f64, t17544: f64, t17547: f64, t17550: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17553 = 4.0_f64 / 45.0_f64 * t2002 * t5365;
    let t17555 = 4.0_f64 / 15.0_f64 * t2002 * t5350;
    let t17557 = t4754 * t815 / 15.0_f64;
    let t17559 = 2.0_f64 / 15.0_f64 * t1887 * t1874;
    let t17561 = t802 * t4682 / 15.0_f64;
    let t17562 = t17527 + t17530 - t17532 + t17534 - t17537 - t17542 + 0.21642082724729686_f64 * t17544 + 0.21642082724729686_f64 * t17547 + 0.011181742741110338_f64 * t17550 + t17553 + t17555 + t17557 + t17559 + t17561;
    (t17553, t17555, t17557, t17559, t17561, t17562)
}
