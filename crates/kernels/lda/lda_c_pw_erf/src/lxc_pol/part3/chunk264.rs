//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 264/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk264<F: Float>(t503: F, t789: F, t11: F, t502: F, t173: F, t184: F) -> (F, F, F, F, F) {
    let t790 = t503 * t789;
    let t791 = t11 * t790;
    let t793 = t502 + F::new(0.0018891666666666666) * t791;
    let t794 = t173 * t793;
    let t795 = t794 * t184;
    (t790, t791, t793, t794, t795)
}
