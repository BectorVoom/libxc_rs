//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 421/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk421<F: Float>(t1960: F, t221: F, t325: F, t790: F, t1245: F, t739: F, t348: F) -> (F, F, F, F) {
    let t1962 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t1960 * t221;
    let t1964 = t325 * t790;
    let t1966 = t1245 * t739;
    let t1967 = t1966 * t348;
    (t1962, t1964, t1966, t1967)
}
