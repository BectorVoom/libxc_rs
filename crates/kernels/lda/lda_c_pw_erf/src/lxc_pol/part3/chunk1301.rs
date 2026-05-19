//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1301/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1301<F: Float>(t11159: F, t11160: F, t11162: F, t11164: F, t11166: F, t11168: F, t13465: F, t13466: F, t13467: F, t13469: F, t13471: F, t13475: F, t13477: F) -> F {
    let t15092 = t11159 + F::new(2.0) / F::new(9.0) * t11160 + F::new(4.0) / F::new(3.0) * t11162 - F::new(2.0) / F::new(9.0) * t11164 - F::new(2.0) / F::new(3.0) * t11166 - F::cast_from(0.040518518518518516_f64) * t11168 - t13465 + t13466 - t13467 + t13469 + t13471 - t13475 - t13477;
    t15092
}
