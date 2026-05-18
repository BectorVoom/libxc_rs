//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 844/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk844<F: Float>(t3494: F, t365: F, t8228: F, t348: F, t361: F, t20: F, t369: F, t3501: F, t3502: F, t642: F, t3509: F, t3510: F) -> (F, F, F, F, F) {
    let t8229 = t365 * t3494 * t8228;
    let t8232 = t348 * t361 * t8228;
    let t8245 = F::new(1.0) / t369 / t20;
    let t8263 = F::new(15.589466666666667) * t3501 * t3502 * t642;
    let t8266 = F::new(2.6116266666666665) * t3509 * t3510 * t642;
    (t8229, t8232, t8245, t8263, t8266)
}
