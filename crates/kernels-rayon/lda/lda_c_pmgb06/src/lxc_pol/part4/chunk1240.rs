//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1240/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1240(t131: f64, t16332: f64, t178: f64, t44: f64, t513: f64, t6688: f64, t12447: f64, t12449: f64, t2002: f64, t4780: f64, t224: f64, t6704: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16336 = t16332 * t44 * t131 * t178 / 30.0_f64;
    let t16338 = t6688 * t513 / 15.0_f64;
    let t16339 = 4.0_f64 / 135.0_f64 * t12447;
    let t16340 = 4.0_f64 / 135.0_f64 * t12449;
    let t16342 = 4.0_f64 / 45.0_f64 * t2002 * t4780;
    let t16343 = t6704 * t224;
    (t16336, t16338, t16339, t16340, t16342, t16343)
}
