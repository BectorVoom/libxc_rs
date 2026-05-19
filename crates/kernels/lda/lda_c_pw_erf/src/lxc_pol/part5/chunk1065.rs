//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1065/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1065<F: Float>(t43: F, t1781: F, t19994: F, t19997: F, t20007: F, t348: F, t4352: F, t4355: F, t47: F, t5982: F, t5992: F, t7354: F, t7360: F, t8315: F, t939: F, t943: F, zeta_threshold: F) -> F {
    let t44 = t43 <= zeta_threshold;
    let t20011 = piecewise3::<F>(t44, F::new(0.0), F::new(40.0) / F::new(81.0) * t8315 * t7354 * t348 - F::new(16.0) / F::new(9.0) * t5982 * t943 - F::new(8.0) / F::new(9.0) * t4352 * t19994 + F::new(8.0) / F::new(3.0) * t4355 * t19997 + F::new(4.0) / F::new(3.0) * t1781 * t5992 + F::new(4.0) / F::new(9.0) * t939 * t7360 * t348 + F::new(4.0) / F::new(3.0) * t47 * t20007);
    t20011
}
