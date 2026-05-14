//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 853/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk853<F: Float>(t2079: F, t955: F, t1554: F, t161: F, t2094: F, t199: F, t5575: F, t2174: F, t566: F, t1139: F, t868: F, t1808: F, t718: F, t122: F, t1669: F, t2116: F) -> (F, F, F, F, F, F, F) {
    let t14183 = t955 * t2079;
    let t14211 = t161 * t1554 * t2094;
    let t14212 = t14211 / 45.0;
    let t14231 = t5575 * t199;
    let t14232 = 0.5025769232130264 * t14231;
    let t14233 = t2174 * t566;
    let t14234 = 0.5025769232130264 * t14233;
    let t14235 = t1139 * t868;
    let t14236 = 0.5025769232130264 * t14235;
    let t14237 = t718 * t1808;
    let t14238 = 0.5025769232130264 * t14237;
    let t14245 = t122 * t1669 * t2116;
    (t14183, t14212, t14232, t14234, t14236, t14238, t14245)
}
