//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1205/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1205<F: Float>(t13430: F, t13434: F, t13437: F, t13442: F, t13446: F, t13452: F, t13457: F, t13464: F, t1635: F, t7266: F, t1627: F, t1: F, t3: F, t6039: F, t604: F, t10015: F, t6759: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t17806 = 32.0 / 81.0 * t13430;
    let t17807 = 64.0 / 135.0 * t13434;
    let t17808 = 128.0 / 135.0 * t13437;
    let t17809 = 64.0 / 81.0 * t13442;
    let t17810 = 64.0 / 135.0 * t13446;
    let t17811 = 128.0 / 135.0 * t13452;
    let t17812 = 64.0 / 81.0 * t13457;
    let t17813 = 8.0 / 45.0 * t13464;
    let t17814 = t7266 * t1635;
    let t17816 = t7266 * t1627;
    let t17820 = t6039 * t1 * t3 * t604;
    let t17823 = 32.0 / 45.0 * t10015 * t6759;
    (t17806, t17807, t17808, t17809, t17810, t17811, t17812, t17813, t17814, t17816, t17820, t17823)
}
