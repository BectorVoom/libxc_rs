//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 233/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk233<F: Float>(t123: F, t199: F, t722: F, t125: F, t398: F, t290: F, t395: F, t100: F, t394: F) -> (F, F, F, F) {
    let t725 = F::cast_from(0.053059442957798957_f64) * t123 * t722 * t199;
    let t726 = t125 * t398;
    let t734 = F::cast_from(0.10665013548435875_f64) * t395 * t290;
    let t740 = F::new(1.0) / t100 / t394;
    (t725, t726, t734, t740)
}
