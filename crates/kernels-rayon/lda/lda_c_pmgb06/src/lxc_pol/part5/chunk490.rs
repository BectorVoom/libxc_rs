//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 490/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk490(t2470: f64, t493: f64, t1972: f64, t835: f64, t2002: f64, t806: f64, t1962: f64, t805: f64) -> (f64, f64, f64, f64) {
    let t2472 = t493 * t2470 / 27.0_f64;
    let t2474 = 2.0_f64 / 45.0_f64 * t1972 * t835;
    let t2476 = 2.0_f64 / 45.0_f64 * t2002 * t806;
    let t2477 = t1962 * t805;
    (t2472, t2474, t2476, t2477)
}
