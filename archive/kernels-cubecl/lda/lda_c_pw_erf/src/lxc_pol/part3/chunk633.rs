//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 633/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk633<F: Float>(t3738: F, t522: F, t519: F, t1523: F, t518: F) -> (F, F, F) {
    let t3739 = t522 * t3738;
    let t3741 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t519 * t3739;
    let t3742 = t1523 * t518;
    (t3739, t3741, t3742)
}
