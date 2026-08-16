//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 945/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk945(t13560: f64, t2085: f64, t2060: f64, t848: f64, t2082: f64, t955: f64, t2079: f64, t1554: f64, t161: f64, t2094: f64, t199: f64, t5575: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14162 = t13560 * t2085;
    let t14170 = t2060 * t848;
    let t14181 = t955 * t2082;
    let t14183 = t955 * t2079;
    let t14211 = t161 * t1554 * t2094;
    let t14212 = t14211 / 45.0_f64;
    let t14231 = t5575 * t199;
    (t14162, t14170, t14181, t14183, t14212, t14231)
}
