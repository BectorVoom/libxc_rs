//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 914/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk914<F: Float>(t197: F, t3892: F, t3518: F, t11857: F, t4488: F, t1390: F, t1440: F, t5127: F, t519: F, t542: F, t9359: F, t9361: F, t9366: F, t9369: F, t3556: F, t795: F) -> (F, F, F, F, F, F, F, F) {
    let t12030 = t3892 * t197;
    let t12031 = t12030 * t3518;
    let t12034 = 32.0 / 27.0 * t4488 * t12031 * t11857;
    let t12039 = 4.0 / 5.0 * t519 * t1440 * t1390 * t5127 * t542;
    let t12040 = 8.0 / 15.0 * t9359;
    let t12041 = 8.0 / 15.0 * t9361;
    let t12042 = 8.0 / 15.0 * t9366;
    let t12043 = 8.0 / 45.0 * t9369;
    let t12044 = t795 * t3556;
    (t12031, t12034, t12039, t12040, t12041, t12042, t12043, t12044)
}
