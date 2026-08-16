//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1327/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1327(t3038: f64, t497: f64, t5068: f64, t6561: f64, t15274: f64, t529: f64, t6559: f64, t1586: f64, t6560: f64, t16825: f64, t5077: f64, t5084: f64) -> (f64, f64, f64, f64) {
    let t17444 = 8.0_f64 / 45.0_f64 * t5068 * t3038 * t497 * t6561;
    let t17448 = 8.0_f64 / 45.0_f64 * t5068 * t6559 * t15274 * t529;
    let t17452 = 4.0_f64 / 45.0_f64 * t5068 * t6559 * t6560 * t1586;
    let t17455 = 4.0_f64 / 15.0_f64 * t5077 * t5084 * t16825;
    (t17444, t17448, t17452, t17455)
}
