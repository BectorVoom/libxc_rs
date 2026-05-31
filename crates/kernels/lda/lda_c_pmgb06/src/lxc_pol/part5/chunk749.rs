//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 749/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk749<F: Float>(t3505: F, t3513: F, t3515: F, t3517: F, t5801: F, t5808: F, t5813: F, t63: F, t7012: F, t7013: F, t7017: F, t7018: F, t7039: F) -> F {
    let t7041 = t7012 + t5801 + t7013 + t5808 - F::cast_from(1.95872_f64) * t5813 - t7017 - F::cast_from(1.46904_f64) * t63 * t7018 - t3505 + t3513 - t3515 - t3517 + t7039;
    t7041
}
