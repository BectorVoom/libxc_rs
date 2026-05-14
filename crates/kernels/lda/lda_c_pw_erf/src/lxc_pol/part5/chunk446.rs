//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 446/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk446<F: Float>(t2171: F, t525: F, t1446: F, t799: F, t473: F, t521: F) -> (F, F, F) {
    let t2173 = 4.0 / 45.0 * t2171 * t525;
    let t2175 = 4.0 / 45.0 * t1446 * t799;
    let t2176 = t473 * t521;
    (t2173, t2175, t2176)
}
