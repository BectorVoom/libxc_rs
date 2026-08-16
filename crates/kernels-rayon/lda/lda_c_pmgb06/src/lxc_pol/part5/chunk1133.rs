//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1133/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1133(t17283: f64, t17285: f64, t17287: f64, t1966: f64, t2064: f64, t439: f64, t6253: f64, t2462: f64, t5194: f64, t16924: f64, t835: f64, t16382: f64, t806: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20602 = 2.0_f64 / 45.0_f64 * t17283;
    let t20603 = 4.0_f64 / 45.0_f64 * t17285;
    let t20604 = 2.0_f64 / 27.0_f64 * t17287;
    let t20608 = 3.0_f64 / 5.0_f64 * t439 * t1966 * t6253 * t2064;
    let t20609 = t5194 * t2462;
    let t20610 = 4.0_f64 / 45.0_f64 * t20609;
    let t20611 = t16924 * t835;
    let t20612 = 2.0_f64 / 45.0_f64 * t20611;
    let t20613 = t16382 * t806;
    (t20602, t20603, t20604, t20608, t20610, t20612, t20613)
}
