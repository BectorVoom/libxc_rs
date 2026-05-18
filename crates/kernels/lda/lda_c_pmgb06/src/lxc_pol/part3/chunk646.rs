//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 646/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk646<F: Float>(t1035: F, t1043: F, t1041: F, t632: F, t1180: F, t242: F, t30: F, t3667: F, t633: F, t409: F, t621: F, t138: F, t634: F) -> (F, F, F, F, F, F, F) {
    let t3868 = t1035 * t1043;
    let t3871 = F::new(48.245938496077606) * t1041 * t3868 * t632;
    let t3874 = F::new(0.0034450798614814814) * t30 * t1180 * t242;
    let t3875 = t3667 * t633;
    let t3877 = F::new(6.0) * t1041 * t3875;
    let t3878 = t409 * t621;
    let t3881 = F::new(0.07123333333333333) * t138 * t3878 * t634;
    (t3868, t3871, t3874, t3875, t3877, t3878, t3881)
}
