//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1118/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1118(t1554: f64, t161: f64, t2094: f64, t486: f64, t4948: f64, t199: f64, t5575: f64, t2174: f64, t566: f64, t1139: f64, t868: f64, t1808: f64, t718: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14211 = t161 * t1554 * t2094;
    let t14213 = t486 * t4948;
    let t14231 = t5575 * t199;
    let t14233 = t2174 * t566;
    let t14235 = t1139 * t868;
    let t14237 = t718 * t1808;
    (t14211, t14213, t14231, t14233, t14235, t14237)
}
