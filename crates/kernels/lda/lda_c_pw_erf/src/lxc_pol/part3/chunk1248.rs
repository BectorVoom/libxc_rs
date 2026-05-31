//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1248/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1248<F: Float>(t10: F, t14668: F, t14807: F, t14814: F, t14817: F, t14819: F, t14822: F, t14837: F, t1568: F, t1664: F, t1856: F, t3251: F, t411: F, t426: F, t5565: F, t5578: F, t767: F) -> F {
    let t14839 = F::cast_from(6.0_f64) * t14807 - F::cast_from(18.0_f64) * t426 * t10 * t5578 * t1664 + t14814 + t14817 - F::cast_from(3.0_f64) * t14819 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t14822 + F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t426 * t10 * t5565 * t411 + F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t426 * t10 * t1856 * t1568 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t426 * t10 * t767 * t3251 - F::cast_from(17.62848_f64) * t14837 + t14668;
    t14839
}
