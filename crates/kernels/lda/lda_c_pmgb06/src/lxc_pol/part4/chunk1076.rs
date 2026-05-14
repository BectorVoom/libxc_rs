//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1076/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1076<F: Float>(t132: F, t137: F, t1395: F, t6225: F, t161: F, t489: F, t6448: F, t1848: F, t2095: F, t4945: F, t831: F, t11952: F, t2648: F, t3058: F, t11971: F, t12022: F) -> (F, F, F, F, F, F, F, F) {
    let t16087 = t132 * t137 * t1395 * t6225 / 15.0;
    let t16089 = t161 * t489 * t6448;
    let t16090 = 4.0 / 45.0 * t16089;
    let t16092 = 2.0 / 15.0 * t1848 * t2095;
    let t16094 = t831 * t4945 / 15.0;
    let t16095 = 8.0 / 135.0 * t11952;
    let t16099 = t132 * t137 * t3058 * t2648 / 30.0;
    let t16100 = 4.0 / 135.0 * t11971;
    let t16101 = 8.0 / 135.0 * t12022;
    (t16087, t16090, t16092, t16094, t16095, t16099, t16100, t16101)
}
