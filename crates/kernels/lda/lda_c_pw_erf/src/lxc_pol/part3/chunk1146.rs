//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1146/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1146<F: Float>(t10326: F, t10350: F, t10361: F, t10654: F, t1318: F, t2001: F, t3854: F, t5405: F, t2171: F, t3808: F, t1472: F, t4788: F) -> (F, F, F, F, F, F, F) {
    let t13415 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t10326;
    let t13416 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t10350;
    let t13417 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t10361;
    let t13419 = t1318 * t10654 * t2001;
    let t13420 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t13419;
    let t13422 = t1318 * t3854 * t5405;
    let t13423 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t13422;
    let t13425 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t2171 * t3808;
    let t13426 = t1472 * t4788;
    (t13415, t13416, t13417, t13420, t13423, t13425, t13426)
}
