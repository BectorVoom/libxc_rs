//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1024/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1024(t3259: f64, t458: f64, t1435: f64, t1540: f64, t1426: f64, t1592: f64, t3238: f64, t517: f64, t1427: f64, t3213: f64, t1710: f64, t431: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10247 = t3259 * t458;
    let t10255 = t1435 * t1540;
    let t10288 = t1426 * t1592;
    let t10293 = t3238 * t517;
    let t10316 = t3213 * t1427;
    let t10318 = t431 * t1710;
    (t10247, t10255, t10288, t10293, t10316, t10318)
}
