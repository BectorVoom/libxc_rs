//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1194/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1194<F: Float>(t10643: F, t10656: F, t4589: F, t544: F, t14029: F, t14033: F, t14037: F, t14040: F, t14042: F, t14045: F, t14047: F, t14050: F, t14053: F, t14054: F) -> (F, F, F, F) {
    let t14055 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t10643;
    let t14056 = F::cast_from(32.0_f64) / F::cast_from(135.0_f64) * t10656;
    let t14058 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t4589 * t544;
    let t14059 = -t14029 - t14033 + t14037 - t14040 - t14042 - t14045 - t14047 - t14050 + t14053 + t14054 - t14055 - t14056 - t14058;
    (t14055, t14056, t14058, t14059)
}
