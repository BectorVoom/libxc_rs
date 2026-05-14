//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 539/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk539<F: Float>(t1476: F, t2924: F, t36: F, t1464: F, t2912: F, t506: F, t1414: F, t337: F, t1083: F) -> (F, F, F, F, F, F, F) {
    let t2925 = t1476 * t2924;
    let t2926 = t36 * t2925;
    let t2928 = t1464 * t2912;
    let t2929 = t506 * t2928;
    let t2930 = t36 * t2929;
    let t2932 = t1414 * t337;
    let t2933 = t2932 * t1083;
    (t2925, t2926, t2928, t2929, t2930, t2932, t2933)
}
