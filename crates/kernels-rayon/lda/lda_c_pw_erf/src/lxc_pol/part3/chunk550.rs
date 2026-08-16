//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 550/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk550(t153: f64, t1953: f64, t2061: f64, t2869: f64, t39: f64) -> f64 {
    let t2872 = -0.005366666666666666_f64 * t1953 - 0.06068888888888889_f64 * t2061 + 0.01829167760955153_f64 * t39 - 0.0036147222222222223_f64 * t153 * t2869;
    t2872
}
