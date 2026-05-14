//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1144/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1144<F: Float>(t10011: F, t6740: F, t3974: F, t4475: F, t5306: F, t4684: F, t6748: F, t3965: F, t4479: F, t5425: F, t12968: F, t5285: F, t6723: F, t951: F, t13966: F, t4506: F) -> (F, F, F, F, F, F, F) {
    let t16829 = t10011 * t6740;
    let t16830 = 64.0 / 135.0 * t16829;
    let t16833 = 32.0 / 45.0 * t3974 * t4475 * t5306;
    let t16836 = 32.0 / 15.0 * t3974 * t6748 * t4684;
    let t16839 = 32.0 / 45.0 * t3965 * t4479 * t5425;
    let t16842 = 32.0 / 45.0 * t3974 * t12968 * t5285;
    let t16843 = t6723 * t951;
    let t16846 = 16.0 / 15.0 * t4506 * t13966 * t16843;
    (t16830, t16833, t16836, t16839, t16842, t16843, t16846)
}
