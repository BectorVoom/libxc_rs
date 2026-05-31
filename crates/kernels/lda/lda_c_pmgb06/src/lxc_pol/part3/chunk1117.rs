//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1117/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1117<F: Float>(t1420: F, t4620: F, t10082: F, t10083: F, t10085: F, t13257: F, t13258: F, t13260: F, t13262: F, t13264: F, t13266: F, t13268: F, t13270: F) -> (F, F) {
    let t13272 = t1420 * t4620 / F::cast_from(9.0_f64);
    let t13273 = t10082 - F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t10083 + F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t10085 - t13257 - t13258 + t13260 + t13262 + t13264 + t13266 + t13268 + t13270 + t13272;
    (t13272, t13273)
}
