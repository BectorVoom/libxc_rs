//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1034/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1034<F: Float>(t1147: F, t117: F, t123: F, t550: F, t1366: F, t3312: F, t3319: F, t3333: F, t3325: F, t184: F, t186: F, t247: F) -> (F, F, F, F, F) {
    let t10670 = t123 * t1147 * t550 * t117;
    let t10679 = t3312 * t1366;
    let t10681 = t3319 * t3333;
    let t10684 = F::new(0.04472697096444135) * t3325 * t3333;
    let t10687 = F::new(0.004413481481481482) * t184 * t247 * t186;
    (t10670, t10679, t10681, t10684, t10687)
}
