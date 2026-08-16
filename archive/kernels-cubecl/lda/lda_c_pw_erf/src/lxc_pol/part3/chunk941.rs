//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 941/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk941<F: Float>(t1245: F, t187: F, t22: F, t1318: F, t3769: F, t3899: F, t1446: F, t3735: F, t1479: F, t3762: F, t571: F, t1484: F, t155: F) -> (F, F, F, F, F) {
    let t10567 = t22 / t187 / t1245;
    let t10574 = t1318 * t3899 * t3769;
    let t10598 = t1446 * t3735;
    let t10603 = t571 * t3762 * t1479;
    let t10605 = t155 * t1484;
    (t10567, t10574, t10598, t10603, t10605)
}
