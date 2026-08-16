//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 923/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk923<F: Float>(t1234: F, t1508: F, t3556: F, t511: F, t2114: F, t3387: F, t3964: F, t668: F) -> (F, F, F, F) {
    let t9973 = t1508 * t1234;
    let t9975 = t511 * t3556;
    let t9977 = t2114 * t3387;
    let t10011 = t3964 * t668;
    (t9973, t9975, t9977, t10011)
}
