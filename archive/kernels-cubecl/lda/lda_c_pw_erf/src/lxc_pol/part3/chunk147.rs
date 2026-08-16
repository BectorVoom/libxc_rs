//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 147/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk147<F: Float>(t379: F, t75: F, t323: F, t325: F, t329: F, t331: F) -> (F, F) {
    let t380 = t75 * t379;
    let t385 = -F::cast_from(0.8630833333333333_f64) * t323 - F::cast_from(0.301925_f64) * t325 - F::cast_from(0.05501625_f64) * t329 - F::cast_from(0.082785_f64) * t331;
    (t380, t385)
}
