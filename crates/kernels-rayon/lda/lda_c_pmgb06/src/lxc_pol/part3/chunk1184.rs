//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1184/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1184(t14053: f64, t14221: f64, t199: f64, t5575: f64, t2174: f64, t566: f64, t1139: f64, t868: f64, t1808: f64, t718: f64, t4463: f64, t81: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14222 = t14053 + t14221;
    let t14231 = t5575 * t199;
    let t14232 = 0.5025769232130264_f64 * t14231;
    let t14233 = t2174 * t566;
    let t14234 = 0.5025769232130264_f64 * t14233;
    let t14235 = t1139 * t868;
    let t14236 = 0.5025769232130264_f64 * t14235;
    let t14237 = t718 * t1808;
    let t14238 = 0.5025769232130264_f64 * t14237;
    let t14239 = t81 * t4463;
    (t14222, t14232, t14234, t14236, t14238, t14239)
}
