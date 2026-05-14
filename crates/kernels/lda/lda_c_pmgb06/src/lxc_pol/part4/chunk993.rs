//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 993/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk993<F: Float>(t1902: F, t3220: F, t1423: F, t5287: F, t5226: F, t5254: F, t5211: F, t5295: F, t5248: F, t5264: F, t4619: F, t464: F, t1894: F, t3213: F, t5365: F, t486: F, t5102: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t13909 = t3220 * t1902;
    let t13911 = t1423 * t5287;
    let t13913 = t1423 * t5226;
    let t13915 = t1423 * t5254;
    let t13917 = t5211 * t5295;
    let t13920 = t5211 * t5248;
    let t13922 = t5211 * t5264;
    let t13933 = t4619 * t464;
    let t13948 = t3213 * t1894;
    let t13950 = t1423 * t5365;
    let t13971 = t486 * t5102;
    (t13909, t13911, t13913, t13915, t13917, t13920, t13922, t13933, t13948, t13950, t13971)
}
