//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1184/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1184<F: Float>(t14053: F, t14221: F, t199: F, t5575: F, t2174: F, t566: F, t1139: F, t868: F, t1808: F, t718: F, t4463: F, t81: F) -> (F, F, F, F, F, F) {
    let t14222 = t14053 + t14221;
    let t14231 = t5575 * t199;
    let t14232 = F::cast_from(0.5025769232130264_f64) * t14231;
    let t14233 = t2174 * t566;
    let t14234 = F::cast_from(0.5025769232130264_f64) * t14233;
    let t14235 = t1139 * t868;
    let t14236 = F::cast_from(0.5025769232130264_f64) * t14235;
    let t14237 = t718 * t1808;
    let t14238 = F::cast_from(0.5025769232130264_f64) * t14237;
    let t14239 = t81 * t4463;
    (t14222, t14232, t14234, t14236, t14238, t14239)
}
