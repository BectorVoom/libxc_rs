//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 909/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk909<F: Float>(t3794: F, t3860: F, t3675: F, t522: F, t3445: F, t565: F, t2104: F, t3390: F, t1284: F, t3564: F, t514: F, t548: F) -> (F, F, F, F, F, F) {
    let t9340 = t3794 * t3860;
    let t9351 = t522 * t3675;
    let t9359 = t565 * t3445;
    let t9361 = t2104 * t3390;
    let t9366 = t1284 * t3390;
    let t9369 = t548 * t514 * t3564;
    (t9340, t9351, t9359, t9361, t9366, t9369)
}
