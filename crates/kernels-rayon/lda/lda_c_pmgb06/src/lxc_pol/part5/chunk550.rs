//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 550/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk550(t1426: f64, t464: f64, t1436: f64, t1450: f64, t517: f64, t1462: f64, t165: f64, t409: f64) -> (f64, f64, f64, f64, f64) {
    let t2948 = t1426 * t464;
    let t2960 = t1436 * t464;
    let t2979 = t1450 * t517;
    let t2991 = t1462 * t517;
    let t3004 = t409 * t165;
    (t2948, t2960, t2979, t2991, t3004)
}
