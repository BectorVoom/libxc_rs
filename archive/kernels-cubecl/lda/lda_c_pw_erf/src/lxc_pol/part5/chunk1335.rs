//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1335/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1335<F: Float>(t13479: F, t21917: F, t21919: F, t21921: F, t21923: F, t21926: F, t21928: F, t21932: F, t21936: F, t21938: F, t21940: F, t21943: F, t21949: F) -> F {
    let t23280 = t21917 - t21919 - t21921 + t21923 - t21926 + t21928 - t13479 - t21932 + t21936 - t21938 - t21940 - t21943 - t21949;
    t23280
}
