//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 648/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk648<F: Float>(t184: F, t5214: F, t1245: F, t806: F, t1972: F, t3859: F, t519: F, t197: F, t3883: F) -> (F, F, F, F, F) {
    let t5215 = t5214 * t184;
    let t5220 = t806 * t1245;
    let t5234 = t3859 * t1972;
    let t5236 = F::new(32.0) / F::new(135.0) * t519 * t5234;
    let t5237 = t3883 * t197;
    (t5215, t5220, t5234, t5236, t5237)
}
