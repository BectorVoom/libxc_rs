//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 941/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk941<F: Float>(t3267: F, t8998: F, t1691: F, t9002: F, t1678: F, t474: F, t426: F, t435: F, t97: F, t1704: F, t1710: F, t3338: F, t440: F, t131: F, t137: F, t3337: F) -> (F, F, F, F, F, F, F, F) {
    let t9017 = t3267 * t8998;
    let t9019 = t1691 * t9002;
    let t9021 = t474 * t1678;
    let t9022 = t426 * t9021;
    let t9037 = 1.0 / t435 / t97;
    let t9054 = t1704 * t1710;
    let t9059 = t440 * t3338;
    let t9068 = t131 / t3337 / t137;
    (t9017, t9019, t9021, t9022, t9037, t9054, t9059, t9068)
}
