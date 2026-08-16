//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 590/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk590(t123: f64, t290: f64, t317: f64, t4001: f64, t113: f64, t2778: f64, t301: f64, t1147: f64, t701: f64, t1321: f64, t67: f64, t107: f64, t1180: f64) -> (f64, f64, f64, f64, f64) {
    let t4005 = 0.9247854820715865_f64 * t123 * t4001 * t290 * t317;
    let t4027 = 0.006715335817467199_f64 * t2778 * t113 * t301;
    let t4030 = t123 * t1147 * t701 * t317;
    let t4042 = 1.0_f64 / t1321 / t67;
    let t4063 = t107 * t1180 * t701;
    (t4005, t4027, t4030, t4042, t4063)
}
