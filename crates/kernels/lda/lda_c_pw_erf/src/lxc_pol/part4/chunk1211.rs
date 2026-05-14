//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1211/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1211<F: Float>(t13115: F, t4475: F, t4842: F, t16559: F, t4506: F, t593: F, t6728: F, t3965: F, t4479: F, t4749: F, t5260: F, t14014: F, t5221: F, t12314: F, t5162: F, t5167: F) -> (F, F, F, F, F, F, F) {
    let t17943 = 64.0 / 45.0 * t13115 * t4475 * t4842;
    let t17947 = 32.0 / 45.0 * t4506 * t6728 * t16559 * t593;
    let t17950 = 32.0 / 45.0 * t3965 * t4479 * t4749;
    let t17953 = 16.0 / 45.0 * t3965 * t4479 * t5260;
    let t17956 = 16.0 / 27.0 * t3965 * t14014 * t5221;
    let t17958 = 64.0 / 45.0 * t12314 * t5162;
    let t17960 = 32.0 / 27.0 * t12314 * t5167;
    (t17943, t17947, t17950, t17953, t17956, t17958, t17960)
}
