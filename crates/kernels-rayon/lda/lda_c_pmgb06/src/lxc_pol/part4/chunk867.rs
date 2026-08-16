//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 867/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk867(t1972: f64, t1995: f64, t1600: f64, t2623: f64, t529: f64, t1992: f64, t493: f64, t1963: f64, t2002: f64, t165: f64, t842: f64, t1994: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6111 = 2.0_f64 / 15.0_f64 * t1972 * t1995;
    let t6112 = t1600 * t2623;
    let t6113 = t6112 * t529;
    let t6114 = t1992 * t6113;
    let t6116 = t493 * t6114 / 15.0_f64;
    let t6118 = 2.0_f64 / 45.0_f64 * t2002 * t1963;
    let t6119 = t165 * t842;
    let t6120 = t6119 * t1994;
    (t6111, t6112, t6113, t6114, t6116, t6118, t6119, t6120)
}
