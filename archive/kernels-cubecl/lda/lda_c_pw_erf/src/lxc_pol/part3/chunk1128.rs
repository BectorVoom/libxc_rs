//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1128/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1128<F: Float>(t1446: F, t5296: F, t1390: F, t2176: F, t1392: F, t1976: F, t519: F, t5299: F, t2146: F, t3869: F, t3855: F, t4763: F) -> (F, F, F, F, F) {
    let t13201 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t1446 * t5296;
    let t13202 = t2176 * t1390;
    let t13206 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t519 * t13202 * t1976 * t1392;
    let t13208 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t1446 * t5299;
    let t13210 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t2146 * t3869;
    let t13211 = t4763 * t3855;
    (t13201, t13206, t13208, t13210, t13211)
}
