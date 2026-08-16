//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1063/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1063(t2002: f64, t6788: f64, t16184: f64, t1972: f64, t6509: f64, t6268: f64, t6513: f64, t6361: f64, t19712: f64, t19714: f64, t19716: f64, t19718: f64, t19722: f64, t19724: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19726 = 2.0_f64 / 15.0_f64 * t2002 * t6788;
    let t19727 = 2.0_f64 / 15.0_f64 * t16184;
    let t19729 = 8.0_f64 / 27.0_f64 * t1972 * t6509;
    let t19731 = 4.0_f64 / 9.0_f64 * t6268 * t6513;
    let t19733 = 2.0_f64 / 15.0_f64 * t2002 * t6361;
    let t19734 = -t19712 - t19714 - t19716 - t19718 - t19722 - t19724 - t19726 - t19727 + t19729 - t19731 - t19733;
    (t19726, t19727, t19729, t19731, t19733, t19734)
}
