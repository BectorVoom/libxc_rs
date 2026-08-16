//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1031/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1031(t1: f64, t2541: f64, t1830: f64, t506: f64, t1825: f64, t5974: f64, t36: f64, t2389: f64, t4851: f64, t1414: f64, t337: f64, t7300: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19336 = t2541 * t1;
    let t19338 = t1830 * t506 * t19336;
    let t19340 = t1825 * t5974;
    let t19342 = t36 * t506 * t19340;
    let t19344 = t4851 * t2389;
    let t19346 = t1830 * t506 * t19344;
    let t19349 = t1414 * t7300 * t337;
    (t19336, t19338, t19340, t19342, t19344, t19346, t19349)
}
