//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 517/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk517(t2088: f64, t518: f64, t166: f64, t161: f64, t517: f64, t842: f64) -> (f64, f64, f64, f64) {
    let t2089 = t518 * t2088;
    let t2090 = t166 * t2089;
    let t2092 = t161 * t2090 / 30.0_f64;
    let t2093 = t842 * t517;
    (t2089, t2090, t2092, t2093)
}
