//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1263/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1263<F: Float>(t123: F, t722: F, t7375: F, t7113: F, t868: F, t2281: F, t2422: F, t10905: F, t10943: F, t10946: F, t125: F, t14666: F, t14669: F, t14697: F, t1808: F, t18954: F, t18969: F, t18979: F, t19130: F, t199: F, t21827: F, t2285: F, t2415: F, t566: F, t6939: F, t7117: F, t7365: F, t81: F, t912: F) -> F {
    let t22177 = t123 * t722 * t7375;
    let t22197 = t123 * t7113 * t868;
    let t22200 = t123 * t2281 * t2422;
    let t22214 = F::new(0.053059442957798957) * t22177 - F::new(0.09550699732403813) * t123 * t912 * t6939 - F::new(0.09550699732403813) * t123 * t2285 * t2422 + F::new(0.9598512193592288) * t18954 - F::new(0.031835665774679375) * t123 * t125 * t19130 * t199 - F::new(0.031835665774679375) * t123 * t7365 * t566 - F::new(0.09550699732403813) * t123 * t7117 * t868 + F::new(0.15917832887339686) * t22197 + F::new(0.15917832887339686) * t22200 + F::new(1.5564103267621028) * t14666 + F::new(1.5564103267621028) * t14669 + F::new(0.15917832887339686) * t18969 + F::new(0.05332506774217938) * t81 * t21827 - F::new(0.09550699732403813) * t123 * t2415 * t1808 - F::new(1.279801625812305) * t10905 + F::new(0.5188034422540342) * t10943 + F::new(0.5188034422540342) * t10946 - F::new(0.31995040645307626) * t18979 - t14697;
    t22214
}
