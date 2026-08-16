//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 954/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk954(t14699: f64, t123: f64, t2833: f64, t868: f64, t1152: f64, t1808: f64, t642: f64, t902: f64, t2164: f64, t247: f64, t2771: f64, t4351: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14700 = 0.42447554366239165_f64 * t14699;
    let t14702 = t123 * t2833 * t868;
    let t14703 = 0.42447554366239165_f64 * t14702;
    let t14705 = t123 * t1152 * t1808;
    let t14706 = 0.42447554366239165_f64 * t14705;
    let t14707 = t642 * t902;
    let t14709 = t247 * t2164;
    let t14710 = 0.9598512193592288_f64 * t14709;
    let t14758 = t4351 * t2771;
    (t14700, t14703, t14706, t14707, t14710, t14758)
}
