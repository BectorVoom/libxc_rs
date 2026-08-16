//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 696/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk696(t44: f64, t6340: f64, t6352: f64, t2519: f64, t607: f64, t4777: f64, t2500: f64, t2948: f64, t439: f64, t2064: f64, t809: f64, t1385: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6355 = (t6340 / 2.0_f64 + t6352 / 2.0_f64) * t44;
    let t6358 = t2519 * t607;
    let t6360 = 4.0_f64 / 405.0_f64 * t4777;
    let t6361 = t2948 * t2500;
    let t6363 = 2.0_f64 / 45.0_f64 * t439 * t6361;
    let t6364 = t809 * t2064;
    let t6365 = t1385 * t6364;
    (t6355, t6358, t6360, t6361, t6363, t6364, t6365)
}
