//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 764/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk764<F: Float>(t525: F, t5327: F, t2158: F, t3416: F, t1472: F, t2163: F, t1959: F, t518: F) -> (F, F, F, F) {
    let t5329 = 8.0 / 45.0 * t5327 * t525;
    let t5331 = 8.0 / 15.0 * t3416 * t2158;
    let t5333 = 8.0 / 15.0 * t1472 * t2163;
    let t5334 = t1959 * t518;
    (t5329, t5331, t5333, t5334)
}
