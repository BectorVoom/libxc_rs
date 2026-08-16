//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 550/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk550<F: Float>(t153: F, t1953: F, t2061: F, t2869: F, t39: F) -> F {
    let t2872 = -F::cast_from(0.005366666666666666_f64) * t1953 - F::cast_from(0.06068888888888889_f64) * t2061 + F::cast_from(0.01829167760955153_f64) * t39 - F::cast_from(0.0036147222222222223_f64) * t153 * t2869;
    t2872
}
