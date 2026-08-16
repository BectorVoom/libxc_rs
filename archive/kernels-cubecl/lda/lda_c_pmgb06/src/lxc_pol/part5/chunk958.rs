//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 958/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk958<F: Float>(t1065: F, t2395: F, t248: F, t6037: F, t980: F, t6068: F, t638: F, t643: F, t6070: F, t1101: F, t2396: F, t1105: F) -> (F, F, F, F, F, F, F) {
    let t14984 = t248 * t2395 * t1065;
    let t15015 = t6037 * t980;
    let t15026 = t638 * t6068;
    let t15028 = t643 * t6068;
    let t15030 = t643 * t6070;
    let t15045 = t1101 * t2396;
    let t15054 = t1105 * t2396;
    (t14984, t15015, t15026, t15028, t15030, t15045, t15054)
}
