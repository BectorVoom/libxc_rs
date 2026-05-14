//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 863/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk863<F: Float>(t1830: F, t2226: F, t2186: F, t1180: F, t776: F, t360: F, t5793: F, t947: F, t2060: F, t5796: F, t5799: F, t5802: F, t2221: F, t410: F, t138: F, t53: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t11388 = t2226 * t1830;
    let t11390 = t2186 * t1830;
    let t11391 = 1.5156425925925925 * t11390;
    let t11392 = t1180 * t776;
    let t11393 = t360 * t11392;
    let t11395 = t5793 * t947;
    let t11396 = 2.93808 * t11395;
    let t11398 = t5796 * t2060;
    let t11400 = t5799 * t947;
    let t11401 = 1.9486833333333333 * t11400;
    let t11402 = t5802 * t2060;
    let t11403 = 1.2991222222222223 * t11402;
    let t11404 = t410 * t2221;
    let t11405 = t360 * t11404;
    let t11406 = 2.0 / 3.0 * t11405;
    let t11407 = t53 * t138;
    (t11388, t11391, t11392, t11393, t11396, t11398, t11401, t11403, t11404, t11406, t11407)
}
