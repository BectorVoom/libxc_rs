//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 927/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk927<F: Float>(t3325: F, t3333: F, t184: F, t186: F, t247: F, t187: F, t3024: F, t3015: F, t3389: F, t534: F, t540: F, t3018: F) -> (F, F, F, F, F, F, F) {
    let t10684 = F::new(0.04472697096444135) * t3325 * t3333;
    let t10687 = F::new(0.004413481481481482) * t184 * t247 * t186;
    let t10690 = F::new(16.0) / F::new(3.0) * t3024 * t187;
    let t10691 = t3015 * t187;
    let t10693 = t534 * t3389;
    let t10696 = F::new(0.004413481481481482) * t540 * t3389;
    let t10697 = t3018 * t187;
    (t10684, t10687, t10690, t10691, t10693, t10696, t10697)
}
