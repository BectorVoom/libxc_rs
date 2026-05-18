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
    let t4366 = piecewise3::<f64>(t44, F::new(0.0), -F::new(8.0) / F::new(27.0) * t4352 * t940 + F::new(16.0) / F::new(9.0) * t4355 * t4356 + F::new(4.0) / F::new(9.0) * t1781 * t945 + F::new(8.0) / F::new(3.0) * t47 * t462 - F::new(8.0) * t1784 * t39);
    let t4367 = t2966 * t743;
    (t4356, t4366, t4367)
}
