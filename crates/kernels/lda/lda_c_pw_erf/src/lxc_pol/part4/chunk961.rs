//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 961/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk961<F: Float>(t1484: F, t155: F, t1487: F, t571: F, t1340: F, t3783: F, t519: F, t219: F, t3762: F) -> (F, F, F, F) {
    let t10605 = t155 * t1484;
    let t10607 = t571 * t10605 * t1487;
    let t10620 = t519 * t3783 * t1340;
    let t10654 = t3762 * t219;
    (t10605, t10607, t10620, t10654)
}
