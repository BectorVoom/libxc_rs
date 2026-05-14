//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 581/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk581<F: Float>(t123: F, t199: F, t4429: F, t395: F, t2799: F, t1156: F, t868: F, t1808: F, t722: F, t1798: F, t315: F, t2281: F, t566: F, t247: F, t902: F, t2142: F, t686: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4431 = t123 * t4429 * t199;
    let t4433 = 2.0 * t395;
    let t4434 = 6.0 * t2799;
    let t4441 = 0.10611888591559791 * t123 * t1156 * t868;
    let t4444 = 0.10611888591559791 * t123 * t722 * t1808;
    let t4454 = t315 * t1798;
    let t4457 = 0.10611888591559791 * t123 * t4454 * t199;
    let t4460 = 0.10611888591559791 * t123 * t2281 * t566;
    let t4461 = 4.0 * t395;
    let t4462 = 12.0 * t2799;
    let t4472 = t247 * t902;
    let t4481 = t2142 * t686;
    (t4431, t4433, t4434, t4441, t4444, t4454, t4457, t4460, t4461, t4462, t4472, t4481)
}
