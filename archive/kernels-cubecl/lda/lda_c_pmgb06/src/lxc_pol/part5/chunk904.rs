//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 904/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk904<F: Float>(t123: F, t199: F, t4297: F, t642: F, t701: F, t10793: F, t2822: F, t566: F, t247: F, t4344: F, t749: F, t327: F) -> (F, F, F, F, F, F) {
    let t10902 = F::cast_from(2.4210827305188265_f64) * t123 * t4297 * t199;
    let t10905 = t642 * t701;
    let t10943 = t123 * t10793 * t199;
    let t10946 = t123 * t2822 * t566;
    let t10967 = t247 * t749 * t4344;
    let t10970 = t327 * t327;
    (t10902, t10905, t10943, t10946, t10967, t10970)
}
