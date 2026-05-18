//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1192/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1192<F: Float>(t11380: F, t11401: F, t11519: F, t11521: F, t18829: F, t18831: F, t18837: F, t21403: F, t21409: F, t21414: F, t21416: F, t21423: F, t8339: F) -> F {
    let t21590 = F::new(1.724255) * t18829 + F::new(6.89702) * t18831 - F::new(2.2990066666666666) * t18837 - F::new(2.2990066666666666) * t11519 + F::new(5.364348888888889) * t11521 + t11380 + t21403 - t11401 - t21409 - t21414 - t8339 + t21416 - t21423;
    t21590
}
