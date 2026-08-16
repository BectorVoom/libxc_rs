//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1015/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1015(t1447: f64, t4762: f64, t1420: f64, t4767: f64, t1594: f64, t1966: f64, t2064: f64, t3031: f64, t439: f64, t1423: f64, t5198: f64, t4766: f64, t5197: f64) -> (f64, f64, f64, f64, f64) {
    let t12075 = t1447 * t4762;
    let t12076 = 2.0_f64 / 5.0_f64 * t12075;
    let t12078 = 3.0_f64 / 5.0_f64 * t1420 * t4767;
    let t12083 = 3.0_f64 / 5.0_f64 * t439 * t1966 * t3031 * t2064 * t1594;
    let t12084 = t1423 * t5198;
    let t12085 = 4.0_f64 / 15.0_f64 * t12084;
    let t12088 = 3.0_f64 / 5.0_f64 * t439 * t5197 * t4766;
    (t12076, t12078, t12083, t12085, t12088)
}
