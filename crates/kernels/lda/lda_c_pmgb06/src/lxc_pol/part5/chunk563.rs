//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 563/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk563<F: Float>(t1366: F, t1372: F, t186: F, t315: F, t934: F, t1375: F, t526: F, t955: F, t163: F, t497: F, t147: F, t740: F) -> (F, F, F, F, F, F, F) {
    let t3331 = F::new(0.21642082724729686) * t1372 * t1366;
    let t3333 = t934 * t315 * t186;
    let t3335 = F::new(0.011181742741110338) * t1375 * t3333;
    let t3350 = t955 * t526;
    let t3357 = F::new(1.0) / t163 / t497;
    let t3358 = t147 * t3357;
    let t3365 = t740 * t147;
    (t3331, t3333, t3335, t3350, t3357, t3358, t3365)
}
