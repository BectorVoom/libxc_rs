//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1037/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1037<F: Float>(t161: F, t166: F, t176: F, t19375: F, t19414: F, t1848: F, t2555: F, t6833: F, t831: F, t15519: F, t15521: F, t15523: F) -> (F, F, F, F, F, F) {
    let t19419 = t161 * t166 * (t19375 + t19414) * t176 / F::new(30.0);
    let t19421 = t1848 * t2555 / F::new(10.0);
    let t19423 = t831 * t6833 / F::new(10.0);
    let t19424 = F::new(4.0) / F::new(45.0) * t15519;
    let t19425 = F::new(8.0) / F::new(45.0) * t15521;
    let t19426 = F::new(4.0) / F::new(27.0) * t15523;
    (t19419, t19421, t19423, t19424, t19425, t19426)
}
