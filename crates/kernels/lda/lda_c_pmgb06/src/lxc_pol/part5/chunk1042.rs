//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1042/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1042<F: Float>(t11380: F, t11401: F, t11519: F, t11521: F, t18829: F, t18831: F, t18837: F, t21403: F, t21409: F, t21414: F, t21416: F, t21423: F, t8339: F, t18848: F, t18851: F, t21411: F, t21439: F, t21442: F, t21445: F, t21448: F, t21451: F, t21461: F, t21462: F, t21463: F, t21465: F, t21466: F, t21477: F, t2247: F) -> (F, F) {
    let t21590 = 1.724255 * t18829 + 6.89702 * t18831 - 2.2990066666666666 * t18837 - 2.2990066666666666 * t11519 + 5.364348888888889 * t11521 + t11380 + t21403 - t11401 - t21409 - t21414 - t8339 + t21416 - t21423;
    let t21595 = 20.69106 * t18848 - 10.34553 * t18851 - 62.07318 * t2247 * t21411 + t21439 - t21442 + t21445 + t21448 + t21451 - t21461 + t21462 + t21463 + t21465 - t21466 - t21477;
    (t21590, t21595)
}
