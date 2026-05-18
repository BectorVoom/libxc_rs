//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 754/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk754<F: Float>(t2414: F, t315: F, t123: F, t199: F, t125: F, t6716: F, t2415: F, t4252: F, t4254: F, t4257: F, t4427: F, t4431: F, t4441: F, t4444: F, t4457: F, t4460: F, t566: F) -> (F, F, F, F) {
    let t7113 = t315 * t2414;
    let t7115 = t123 * t7113 * t199;
    let t7117 = t125 * t6716;
    let t7124 = t4252 - F::new(0.14149184788746388) * t4254 - F::new(0.14149184788746388) * t4257 - F::new(0.28298369577492777) * t4431 + t4457 + t4460 - F::new(0.28298369577492777) * t4427 + t4441 + t4444 + F::new(0.053059442957798957) * t7115 - F::new(0.031835665774679375) * t123 * t7117 * t199 - F::new(0.031835665774679375) * t123 * t2415 * t566;
    (t7113, t7115, t7117, t7124)
}
