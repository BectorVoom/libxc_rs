//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 931/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk931(t3247: f64, t5065: f64, t5066: f64, t2911: f64, t518: f64, t27: f64, t409: f64, t1461: f64, t1464: f64, t177: f64, t12514: f64, t441: f64, t5075: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12529 = t5065 * t5066 * t3247;
    let t12530 = t518 * t2911;
    let t12535 = t27 * t409;
    let t12537 = t5065 * t12535 * t1461;
    let t12546 = t177 * t1464;
    let t12555 = t5075 * t12514 * t441;
    (t12529, t12530, t12535, t12537, t12546, t12555)
}
