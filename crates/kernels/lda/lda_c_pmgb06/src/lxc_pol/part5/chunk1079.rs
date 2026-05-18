//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1079/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1079<F: Float>(t16298: F, t16305: F, t16307: F, t16309: F, t16314: F, t188: F, t539: F, t7364: F, t16350: F, t10687: F, t10690: F, t12448: F, t12450: F) -> (F, F, F, F, F, F, F) {
    let t19975 = t16298 / F::new(15.0);
    let t19976 = t16305 / F::new(45.0);
    let t19977 = F::new(2.0) / F::new(81.0) * t16307;
    let t19978 = F::new(4.0) / F::new(15.0) * t16309;
    let t19979 = F::new(4.0) / F::new(135.0) * t16314;
    let t19981 = t7364 * t539 * t188;
    let t19983 = F::new(4.0) / F::new(135.0) * t16350;
    let t19984 = -t19975 - t19976 - t10687 + t10690 - t19977 + t19978 - t19979 - t12448 - t12450 + F::new(4.0) / F::new(3.0) * t19981 + t19983;
    (t19975, t19976, t19977, t19978, t19979, t19983, t19984)
}
