//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1157/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1157<F: Float>(t2561: F, t3762: F, t571: F, t12136: F, t12684: F, t12693: F, t12697: F, t12708: F, t14043: F, t1446: F, t1472: F, t1513: F, t16847: F, t16848: F, t2473: F, t3794: F, t4488: F, t4494: F, t4576: F, t4724: F, t493: F, t5143: F, t5148: F, t542: F, t6710: F, t6905: F, t6946: F, t6970: F, t6974: F, t795: F, t822: F) -> (F,) {
    let t17040 = t571 * t3762 * t2561;
    let t17050 = 16.0 / 45.0 * t795 * t4724 + 8.0 / 15.0 * t1513 * t2473 - 32.0 / 135.0 * t12684 - 8.0 / 15.0 * t493 * t14043 - 16.0 / 45.0 * t822 * t4576 - 64.0 / 135.0 * t12693 + 64.0 / 135.0 * t12697 - 64.0 / 405.0 * t12708 + 32.0 / 45.0 * t4488 * t6710 * t16847 * t542 + 32.0 / 45.0 * t4488 * t4494 * t16848 - 64.0 / 45.0 * t12136 * t5143 + 32.0 / 27.0 * t12136 * t5148 + 16.0 / 405.0 * t17040 + 16.0 / 15.0 * t3794 * t6946 - 8.0 / 5.0 * t1472 * t6970 + 16.0 / 15.0 * t1472 * t6974 - 8.0 / 5.0 * t1446 * t6905;
    (t17050,)
}
