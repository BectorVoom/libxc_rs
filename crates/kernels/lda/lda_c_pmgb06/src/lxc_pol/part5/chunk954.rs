//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 954/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk954<F: Float>(t16314: F, t188: F, t539: F, t7364: F, t16350: F, t10687: F, t10690: F, t12448: F, t12450: F, t19975: F, t19976: F, t19977: F, t19978: F, t12456: F, t12465: F, t16380: F) -> (F, F, F, F, F, F) {
    let t19979 = 4.0 / 135.0 * t16314;
    let t19981 = t7364 * t539 * t188;
    let t19983 = 4.0 / 135.0 * t16350;
    let t19984 = -t19975 - t19976 - t10687 + t10690 - t19977 + t19978 - t19979 - t12448 - t12450 + 4.0 / 3.0 * t19981 + t19983;
    let t19985 = 4.0 / 135.0 * t12456;
    let t19986 = 4.0 / 135.0 * t12465;
    let t19987 = 4.0 / 45.0 * t16380;
    (t19979, t19983, t19984, t19985, t19986, t19987)
}
