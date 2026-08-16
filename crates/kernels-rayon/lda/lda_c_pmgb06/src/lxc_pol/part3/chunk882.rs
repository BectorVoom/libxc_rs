//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 882/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk882(t1200: f64, t1329: f64, t199: f64, t3982: f64, t1139: f64, t566: f64, t107: f64, t2786: f64, t701: f64, t290: f64, t8170: f64, t3076: f64, t432: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9052 = t1329 * t1200;
    let t9061 = t3982 * t199;
    let t9063 = t1139 * t566;
    let t9066 = t107 * t2786 * t701;
    let t9070 = 19.1926369973667_f64 * t107 * t8170 * t290;
    let t9089 = t432 * t3076;
    (t9052, t9061, t9063, t9066, t9070, t9089)
}
