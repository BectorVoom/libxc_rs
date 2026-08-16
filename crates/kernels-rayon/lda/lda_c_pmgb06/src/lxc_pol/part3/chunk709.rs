//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 709/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk709(t1454: f64, t1988: f64, t1461: f64, t842: f64, t1466: f64, t1447: f64, t1995: f64, t1980: f64, t485: f64) -> (f64, f64, f64, f64, f64) {
    let t4585 = t1988 * t1454;
    let t4588 = t1461 * t842;
    let t4589 = t4588 * t1466;
    let t4593 = 4.0_f64 / 45.0_f64 * t1447 * t1995;
    let t4602 = t485 * t1980;
    (t4585, t4588, t4589, t4593, t4602)
}
