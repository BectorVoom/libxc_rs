//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 946/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk946<F: Float>(t123: F, t2281: F, t868: F, t2422: F, t722: F, t2407: F, t395: F, t1808: F, t2285: F, t305: F, t4283: F, t4284: F, t4472: F, t4579: F, t6104: F, t6939: F, t726: F, t81: F, t912: F) -> F {
    let t7126 = t123 * t2281 * t868;
    let t7135 = t123 * t722 * t2422;
    let t7145 = t395 * t2407;
    let t7149 = F::new(0.10611888591559791) * t7126 - F::new(0.06367133154935875) * t123 * t2285 * t868 - F::new(0.06367133154935875) * t123 * t912 * t1808 + F::new(0.053059442957798957) * t7135 - F::new(0.031835665774679375) * t123 * t726 * t2422 - F::new(0.031835665774679375) * t123 * t305 * t6939 - t4283 + F::new(0.31995040645307626) * t4284 + F::new(0.6399008129061525) * t4472 - t4579 - F::new(0.10665013548435875) * t7145 + F::new(0.05332506774217938) * t81 * t6104;
    t7149
}
