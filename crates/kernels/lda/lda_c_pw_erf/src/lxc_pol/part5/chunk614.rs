//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 614/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk614<F: Float>(t2022: F, t3863: F, t571: F, t1333: F, t833: F, t2026: F, t3859: F, t1325: F, t1981: F, t518: F) -> (F, F, F, F, F, F) {
    let t5302 = t3863 * t2022;
    let t5304 = 16.0 / 135.0 * t571 * t5302;
    let t5305 = t833 * t1333;
    let t5310 = t3859 * t2026;
    let t5312 = 32.0 / 135.0 * t1325 * t5310;
    let t5327 = t1981 * t518;
    (t5302, t5304, t5305, t5310, t5312, t5327)
}
