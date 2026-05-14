//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 814/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk814<F: Float>(t3273: F, t8920: F, t128: F, t1652: F, t19: F, t8990: F, t3267: F, t8998: F, t1691: F, t9002: F, t1678: F, t474: F, t426: F, t156: F, t3223: F, t435: F, t97: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9011 = t3273 * t8920;
    let t9015 = t1652 * t128 * t19 * t8990;
    let t9017 = t3267 * t8998;
    let t9019 = t1691 * t9002;
    let t9021 = t474 * t1678;
    let t9022 = t426 * t9021;
    let t9024 = t156 * t3223;
    let t9025 = t426 * t9024;
    let t9037 = 1.0 / t435 / t97;
    (t9011, t9015, t9017, t9019, t9021, t9022, t9024, t9025, t9037)
}
