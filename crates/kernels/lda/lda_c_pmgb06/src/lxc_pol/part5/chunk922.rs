//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 922/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk922<F: Float>(t161: F, t166: F, t176: F, t19375: F, t19414: F, t1848: F, t2555: F, t6833: F, t831: F, t15519: F, t15521: F, t15523: F, t1423: F, t7555: F, t19307: F, t19309: F, t19311: F, t19313: F) -> (F, F, F, F, F, F, F, F) {
    let t19419 = t161 * t166 * (t19375 + t19414) * t176 / 30.0;
    let t19421 = t1848 * t2555 / 10.0;
    let t19423 = t831 * t6833 / 10.0;
    let t19424 = 4.0 / 45.0 * t15519;
    let t19425 = 8.0 / 45.0 * t15521;
    let t19426 = 4.0 / 27.0 * t15523;
    let t19427 = t1423 * t7555;
    let t19428 = 2.0 / 45.0 * t19427;
    let t19429 = -t19307 + t19309 - t19311 - t19313 + t19419 + t19421 + t19423 - t19424 - t19425 + t19426 - t19428;
    (t19419, t19421, t19423, t19424, t19425, t19426, t19428, t19429)
}
