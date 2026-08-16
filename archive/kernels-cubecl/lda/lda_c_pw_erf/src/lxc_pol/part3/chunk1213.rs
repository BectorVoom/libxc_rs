//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1213/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1213<F: Float>(t2171: F, t3888: F, t1325: F, t1440: F, t2166: F, t3545: F, t10162: F, t2167: F, t3787: F, t5381: F, t3794: F, t4953: F) -> (F, F, F, F, F) {
    let t14307 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t2171 * t3888;
    let t14311 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t1325 * t1440 * t2166 * t3545;
    let t14313 = t1325 * t10162 * t2167;
    let t14314 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t14313;
    let t14316 = t1325 * t3787 * t5381;
    let t14317 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t14316;
    let t14319 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t3794 * t4953;
    (t14307, t14311, t14314, t14317, t14319)
}
