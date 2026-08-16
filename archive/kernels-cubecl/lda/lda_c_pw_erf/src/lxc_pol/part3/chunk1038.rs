//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1038/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1038<F: Float>(t12158: F, t12160: F, t571: F, t1472: F, t4873: F, t1278: F, t1976: F, t4848: F, t519: F, t1987: F, t3709: F, t1446: F, t4856: F) -> (F, F, F, F, F) {
    let t12163 = F::cast_from(64.0_f64) / F::cast_from(27.0_f64) * t571 * t12158 * t12160;
    let t12165 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t1472 * t4873;
    let t12169 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t519 * t4848 * t1976 * t1278;
    let t12171 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t3709 * t1987;
    let t12173 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t1446 * t4856;
    (t12163, t12165, t12169, t12171, t12173)
}
