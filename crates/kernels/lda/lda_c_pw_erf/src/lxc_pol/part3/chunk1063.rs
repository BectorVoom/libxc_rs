//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1063/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1063<F: Float>(t12450: F, t3965: F, t5147: F, t12031: F, t12389: F, t3619: F, t4506: F, t5151: F, t10011: F, t5138: F, t5143: F, t5148: F) -> (F, F, F, F, F, F) {
    let t12453 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t3965 * t5147 * t12450;
    let t12456 = F::cast_from(64.0_f64) / F::cast_from(27.0_f64) * t3965 * t12031 * t12389;
    let t12459 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t4506 * t5151 * t3619;
    let t12460 = t10011 * t5138;
    let t12461 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t12460;
    let t12462 = t10011 * t5143;
    let t12463 = F::cast_from(64.0_f64) / F::cast_from(45.0_f64) * t12462;
    let t12464 = t10011 * t5148;
    (t12453, t12456, t12459, t12461, t12463, t12464)
}
