//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 875/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk875<F: Float>(t379: F, t386: F, t400: F, t8164: F, t1026: F, t1012: F) -> (F, F, F) {
    let t8168 = F::new(0.5848223397455204) * t400 * t379 * t8164 * t386;
    let t8169 = t1026 * t1026;
    let t8170 = F::new(1.0) / t8169;
    let t8171 = t1012 * t1012;
    (t8168, t8170, t8171)
}
