//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 488/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk488(t2462: f64, t493: f64, t2389: f64, t498: f64, t496: f64) -> (f64, f64, f64) {
    let t2464 = 2.0_f64 / 45.0_f64 * t493 * t2462;
    let t2465 = t498 * t2389;
    let t2466 = t496 * t2465;
    (t2464, t2465, t2466)
}
