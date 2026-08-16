//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1091/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1091(t16524: f64, t1423: f64, t7646: f64, t2485: f64, t5220: f64, t7574: f64, t2481: f64, t132: f64, t435: f64, t7502: f64, t16535: f64, t16537: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20127 = 8.0_f64 / 27.0_f64 * t16524;
    let t20128 = t1423 * t7646;
    let t20129 = 4.0_f64 / 27.0_f64 * t20128;
    let t20130 = t5220 * t2485;
    let t20131 = 2.0_f64 / 27.0_f64 * t20130;
    let t20132 = t1423 * t7574;
    let t20133 = 2.0_f64 / 45.0_f64 * t20132;
    let t20134 = t5220 * t2481;
    let t20135 = 2.0_f64 / 45.0_f64 * t20134;
    let t20137 = t132 * t435 * t7502;
    let t20138 = t20137 / 45.0_f64;
    let t20139 = 2.0_f64 / 15.0_f64 * t16535;
    let t20140 = 2.0_f64 / 15.0_f64 * t16537;
    (t20127, t20129, t20131, t20133, t20135, t20138, t20139, t20140)
}
