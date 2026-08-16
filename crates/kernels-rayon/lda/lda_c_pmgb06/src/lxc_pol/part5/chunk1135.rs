//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1135/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1135(t2480: f64, t439: f64, t4779: f64, t1992: f64, t493: f64, t529: f64, t7806: f64, t9636: f64, t2088: f64, t6285: f64, t1966: f64, t477: f64, t7811: f64, t9647: f64) -> (f64, f64, f64, f64) {
    let t20627 = t439 * t4779 * t2480 / 15.0_f64;
    let t20632 = 4.0_f64 / 5.0_f64 * t493 * t1992 * t9636 * t7806 * t529;
    let t20636 = 3.0_f64 / 5.0_f64 * t493 * t1992 * t6285 * t2088;
    let t20641 = 4.0_f64 / 5.0_f64 * t439 * t1966 * t9647 * t7811 * t477;
    (t20627, t20632, t20636, t20641)
}
