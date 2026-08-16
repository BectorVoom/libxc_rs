//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 953/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk953(t301: f64, t413: f64, t5575: f64, t1183: f64, t2174: f64, t123: f64, t2822: f64, t868: f64, t14277: f64, t199: f64, t14281: f64, t4429: f64, t566: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14639 = t5575 * t413 * t301;
    let t14640 = 0.0017434044910732151_f64 * t14639;
    let t14642 = t2174 * t1183 * t301;
    let t14666 = t123 * t2822 * t868;
    let t14669 = t123 * t14277 * t199;
    let t14696 = t123 * t14281 * t199;
    let t14697 = 0.42447554366239165_f64 * t14696;
    let t14699 = t123 * t4429 * t566;
    (t14640, t14642, t14666, t14669, t14697, t14699)
}
