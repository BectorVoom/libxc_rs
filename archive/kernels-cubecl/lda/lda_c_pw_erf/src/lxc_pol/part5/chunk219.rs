//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 219/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk219<F: Float>(t602: F, t604: F, t325: F, t331: F) -> (F, F, F) {
    let pi = F::cast_from(M_PI);
    let t606 = F::cast_from(0.10821041362364843_f64) * t602 * t604;
    let t609 = F::cast_from(0.4125_f64) * t325 - t331 / F::cast_from(6.0_f64);
    let t610 = t609 * pi;
    (t606, t609, t610)
}
