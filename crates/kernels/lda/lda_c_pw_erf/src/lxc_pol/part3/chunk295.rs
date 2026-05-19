//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 295/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk295<F: Float>(t50: F, t52: F, t950: F, t951: F, t954: F, t59: F, t949: F, zeta_threshold: F) -> F {
    let t51 = t50 <= zeta_threshold;
    let t958 = piecewise3::<F>(t51, F::new(0.0), F::new(4.0) / F::new(9.0) * t950 * t951 + F::new(4.0) / F::new(3.0) * t52 * t954);
    let t960 = (t949 + t958) * t59;
    t960
}
