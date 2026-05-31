//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 600/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk600<F: Float>(t1326: F, t3412: F, t519: F, t1283: F, t518: F) -> (F, F, F) {
    let t3413 = t1326 * t3412;
    let t3415 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t519 * t3413;
    let t3416 = t1283 * t518;
    (t3413, t3415, t3416)
}
