//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 558/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk558(t1476: f64, t2924: f64, t36: f64, t1464: f64, t2912: f64, t506: f64, t1414: f64, t337: f64, t1083: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2925 = t1476 * t2924;
    let t2926 = t36 * t2925;
    let t2928 = t1464 * t2912;
    let t2929 = t506 * t2928;
    let t2930 = t36 * t2929;
    let t2932 = t1414 * t337;
    let t2933 = t2932 * t1083;
    (t2925, t2926, t2928, t2929, t2930, t2932, t2933)
}
