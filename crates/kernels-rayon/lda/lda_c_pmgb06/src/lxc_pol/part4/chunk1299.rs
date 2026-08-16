//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1299/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1299(t1825: f64, t1830: f64, t506: f64, t2550: f64, t947: f64, t1: f64, t1840: f64) -> (f64, f64, f64) {
    let t17064 = t1830 * t506 * t1825;
    let t17066 = t947 * t2550;
    let t17070 = t1840 * t1;
    (t17064, t17066, t17070)
}
