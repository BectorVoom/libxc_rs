//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1221/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1221<F: Float>(t11952: F, t132: F, t137: F, t2648: F, t3058: F, t11971: F, t12022: F, t12037: F, t12039: F, t12036: F, t835: F, t2462: F, t3223: F) -> (F, F, F, F, F, F, F, F) {
    let t16095 = F::new(8.0) / F::new(135.0) * t11952;
    let t16099 = t132 * t137 * t3058 * t2648 / F::new(30.0);
    let t16100 = F::new(4.0) / F::new(135.0) * t11971;
    let t16101 = F::new(8.0) / F::new(135.0) * t12022;
    let t16102 = F::new(8.0) / F::new(405.0) * t12037;
    let t16103 = F::new(8.0) / F::new(135.0) * t12039;
    let t16104 = t12036 * t835;
    let t16105 = F::new(4.0) / F::new(405.0) * t16104;
    let t16106 = t3223 * t2462;
    (t16095, t16099, t16100, t16101, t16102, t16103, t16105, t16106)
}
