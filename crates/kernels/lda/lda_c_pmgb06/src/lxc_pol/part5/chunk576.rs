//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 576/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk576<F: Float>(t4238: F, t83: F, t419: F, t1770: F, t1767: F, t398: F, t1186: F, t1768: F, t123: F, t199: F, t2822: F, t2833: F, t1152: F, t566: F, t290: F, t642: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4239 = t4238 * t83;
    let t4240 = t4239 * t419;
    let t4242 = 0.0001639671923854359 * t4240 * t1770;
    let t4243 = t1767 * t398;
    let t4244 = t4243 * t419;
    let t4245 = t4244 * t1770;
    let t4247 = t1768 * t1186;
    let t4249 = 5.4655730795145296e-05 * t4247 * t1770;
    let t4252 = 0.5188034422540342 * t123 * t2822 * t199;
    let t4254 = t123 * t2833 * t199;
    let t4257 = t123 * t1152 * t566;
    let t4283 = 1.279801625812305 * t642 * t290;
    (t4239, t4240, t4242, t4243, t4244, t4245, t4247, t4249, t4252, t4254, t4257, t4283)
}
