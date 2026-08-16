//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1003/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1003(t5381: f64, t588: f64, t97: f64, t1981: f64, t4852: f64, t5447: f64, t4866: f64, t5463: f64, t2983: f64, t493: f64, t5486: f64, t2933: f64, t6747: f64) -> (f64, f64, f64, f64, f64) {
    let t11930 = t5381 * t97 * t588;
    let t11934 = 8.0_f64 / 15.0_f64 * t1981 * t5447 * t4852;
    let t11937 = 4.0_f64 / 9.0_f64 * t1981 * t5463 * t4866;
    let t11940 = t493 * t5486 * t2983 / 15.0_f64;
    let t11943 = 2.0_f64 / 15.0_f64 * t493 * t6747 * t2933;
    (t11930, t11934, t11937, t11940, t11943)
}
