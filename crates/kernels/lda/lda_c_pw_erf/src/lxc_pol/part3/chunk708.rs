//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 708/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk708<F: Float>(t3975: F, t811: F, t1309: F, t3974: F, t3966: F, t784: F, t1314: F, t3965: F, t806: F) -> (F, F, F, F, F, F, F) {
    let t4475 = t3975 * t811;
    let t4476 = t4475 * t1309;
    let t4478 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t3974 * t4476;
    let t4479 = t3966 * t784;
    let t4480 = t4479 * t1314;
    let t4482 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t3965 * t4480;
    let t4483 = t3966 * t806;
    (t4475, t4476, t4478, t4479, t4480, t4482, t4483)
}
