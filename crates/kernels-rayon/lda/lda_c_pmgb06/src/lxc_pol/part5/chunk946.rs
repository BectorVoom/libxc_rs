//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 946/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk946(t14231: f64, t2174: f64, t566: f64, t1139: f64, t868: f64, t1808: f64, t718: f64, t122: f64, t1669: f64, t2116: f64, t421: f64, t5900: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14232 = 0.5025769232130264_f64 * t14231;
    let t14233 = t2174 * t566;
    let t14234 = 0.5025769232130264_f64 * t14233;
    let t14235 = t1139 * t868;
    let t14236 = 0.5025769232130264_f64 * t14235;
    let t14237 = t718 * t1808;
    let t14238 = 0.5025769232130264_f64 * t14237;
    let t14245 = t122 * t1669 * t2116;
    let t14246 = 0.15917832887339686_f64 * t14245;
    let t14275 = t5900 * t421;
    (t14232, t14234, t14236, t14238, t14246, t14275)
}
