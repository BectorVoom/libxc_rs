//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 713/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk713<F: Float>(t125: F, t4463: F, t1167: F, t1200: F, t123: F, t1808: F, t199: F, t2285: F, t305: F, t4427: F, t4431: F, t4435: F, t4441: F, t4444: F, t4457: F, t4460: F, t566: F, t726: F, t868: F, t912: F) -> (F, F) {
    let t4464 = t125 * t4463;
    let t4471 = -F::new(0.14149184788746388) * t4427 - F::new(0.14149184788746388) * t4431 - F::new(0.031835665774679375) * t123 * t305 * t4435 + t4441 + t4444 - F::new(0.031835665774679375) * t123 * t1167 * t868 - F::new(0.06367133154935875) * t123 * t726 * t1808 - F::new(0.031835665774679375) * t123 * t912 * t1200 + t4457 + t4460 - F::new(0.031835665774679375) * t123 * t4464 * t199 - F::new(0.06367133154935875) * t123 * t2285 * t566;
    (t4464, t4471)
}
