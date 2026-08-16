//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1119/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1119<F: Float>(t1318: F, t4688: F, t4758: F, t951: F, t4804: F, t5409: F, t3794: F, t1325: F, t3859: F, t5275: F, t5237: F, t5265: F) -> (F, F, F, F, F) {
    let t13096 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t1318 * t4758 * t4688 * t951;
    let t13097 = t4804 * t5409;
    let t13098 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t13097;
    let t13099 = t3794 * t5409;
    let t13100 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t13099;
    let t13102 = t1325 * t3859 * t5275;
    let t13103 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t13102;
    let t13105 = t1325 * t5237 * t5265;
    (t13096, t13098, t13100, t13103, t13105)
}
