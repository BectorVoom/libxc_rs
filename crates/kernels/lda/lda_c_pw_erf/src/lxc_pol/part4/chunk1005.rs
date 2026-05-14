//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1005/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1005<F: Float>(t4753: F, t5363: F, t504: F, t529: F, t2176: F, t798: F, t519: F, t3794: F, t5378: F, t1472: F, t4901: F, t2143: F, t3709: F, t1446: F, t4907: F, t2171: F, t3784: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12577 = t4753 * t5363;
    let t12600 = t529 * t504;
    let t12615 = t2176 * t798;
    let t12616 = t519 * t12615;
    let t12621 = t3794 * t5378;
    let t12629 = t1472 * t4901;
    let t12631 = t3709 * t2143;
    let t12633 = t1446 * t4907;
    let t12637 = t2171 * t3784;
    (t12577, t12600, t12615, t12616, t12621, t12629, t12631, t12633, t12637)
}
