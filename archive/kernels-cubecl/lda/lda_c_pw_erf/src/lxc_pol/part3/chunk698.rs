//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 698/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk698<F: Float>(t43: F, t348: F, t462: F, t1781: F, t1784: F, t39: F, t4352: F, t4355: F, t47: F, t940: F, t945: F, t2966: F, t743: F, zeta_threshold: F) -> (F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t4356 = t462 * t348;
    let t4366 = piecewise3::<F>(t44, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t4352 * t940 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t4355 * t4356 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1781 * t945 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t47 * t462 - F::cast_from(8.0_f64) * t1784 * t39);
    let t4367 = t2966 * t743;
    (t4356, t4366, t4367)
}
