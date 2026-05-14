//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 687/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk687<F: Float>(t2035: F, t3416: F, t2002: F, t1315: F, t2171: F, t2098: F, t504: F, t348: F, t1313: F, t519: F, t2103: F, t518: F) -> (F, F, F, F, F, F, F, F) {
    let t4743 = 16.0 / 45.0 * t3416 * t2035;
    let t4745 = 16.0 / 45.0 * t3416 * t2002;
    let t4747 = 8.0 / 45.0 * t2171 * t1315;
    let t4748 = t2098 * t504;
    let t4749 = t4748 * t348;
    let t4750 = t1313 * t4749;
    let t4752 = 8.0 / 45.0 * t519 * t4750;
    let t4753 = t2103 * t518;
    (t4743, t4745, t4747, t4748, t4749, t4750, t4752, t4753)
}
