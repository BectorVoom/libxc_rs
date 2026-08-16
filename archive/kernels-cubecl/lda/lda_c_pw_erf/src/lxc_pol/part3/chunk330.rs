//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 330/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk330<F: Float>(t1125: F, t153: F, t462: F, t925: F, t933: F) -> F {
    let t1128 = F::cast_from(0.0023_f64) * t925 + F::cast_from(0.022758333333333332_f64) * t933 - F::cast_from(0.006097225869850511_f64) * t462 + F::cast_from(0.0010844166666666667_f64) * t153 * t1125;
    t1128
}
