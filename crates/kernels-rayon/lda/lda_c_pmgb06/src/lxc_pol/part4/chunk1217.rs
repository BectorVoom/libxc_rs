//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1217/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1217(t2948: f64, t439: f64, t6412: f64, t1385: f64, t477: f64, t6217: f64, t1629: f64, t2578: f64, t2002: f64, t5238: f64, t1447: f64, t6399: f64) -> (f64, f64, f64, f64, f64) {
    let t16040 = 2.0_f64 / 45.0_f64 * t439 * t2948 * t6412;
    let t16044 = 2.0_f64 / 45.0_f64 * t439 * t1385 * t6217 * t477;
    let t16048 = t439 * t1385 * t2578 * t1629 / 45.0_f64;
    let t16050 = 4.0_f64 / 45.0_f64 * t2002 * t5238;
    let t16051 = t1447 * t6399;
    (t16040, t16044, t16048, t16050, t16051)
}
