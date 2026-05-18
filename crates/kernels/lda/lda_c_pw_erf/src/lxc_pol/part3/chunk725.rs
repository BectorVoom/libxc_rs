//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 725/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk725<F: Float>(t1518: F, t834: F, t211: F, t785: F, t493: F, t1: F, t1124: F) -> (F, F, F, F, F) {
    let t4561 = t1518 * t834;
    let t4562 = t211 * t4561;
    let t4563 = F::new(4.0) / F::new(135.0) * t4562;
    let t4564 = t1518 * t785;
    let t4565 = t493 * t4564;
    let t4566 = F::new(8.0) / F::new(135.0) * t4565;
    let t4567 = t1 * t1124;
    (t4561, t4563, t4564, t4566, t4567)
}
