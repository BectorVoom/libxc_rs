//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1029/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1029<F: Float>(t14053: F, t14221: F, t199: F, t5575: F, t2174: F, t566: F, t1139: F, t868: F, t1808: F, t718: F, t4463: F, t81: F, t5522: F, t122: F, t1669: F, t2116: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14222 = t14053 + t14221;
    let t14231 = t5575 * t199;
    let t14232 = 0.5025769232130264 * t14231;
    let t14233 = t2174 * t566;
    let t14234 = 0.5025769232130264 * t14233;
    let t14235 = t1139 * t868;
    let t14236 = 0.5025769232130264 * t14235;
    let t14237 = t718 * t1808;
    let t14238 = 0.5025769232130264 * t14237;
    let t14239 = t81 * t4463;
    let t14240 = t14239 * t199;
    let t14242 = t5522 * t566;
    let t14245 = t122 * t1669 * t2116;
    (t14222, t14232, t14234, t14236, t14238, t14239, t14240, t14242, t14245)
}
