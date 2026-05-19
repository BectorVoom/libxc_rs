//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 19/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk19<F: Float>(t11: F, t14: F, t17: F, t25: F) -> (F, F, F) {
    let t27 = F::new(3.79785) * t14 + F::new(0.8969) * t11 + F::new(0.204775) * t17 + F::new(0.123235) * t25;
    let t30 = F::new(1.0) + F::cast_from(16.081824322151103_f64) / t27;
    let t31 = F::ln(t30);
    (t27, t30, t31)
}
