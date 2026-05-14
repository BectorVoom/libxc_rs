//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 961/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk961<F: Float>(t9941: F, t9944: F, t9947: F, t9949: F, t9953: F, t9973: F, t9975: F, t9977: F, t1440: F, t3675: F, t1325: F, t1392: F, t494: F, t806: F, t542: F, t1278: F, t519: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12757 = 16.0 / 45.0 * t9941;
    let t12758 = 8.0 / 45.0 * t9944;
    let t12759 = 16.0 / 135.0 * t9947;
    let t12760 = 8.0 / 45.0 * t9949;
    let t12761 = 4.0 / 15.0 * t9953;
    let t12762 = 4.0 / 15.0 * t9973;
    let t12763 = 4.0 / 15.0 * t9975;
    let t12764 = 8.0 / 15.0 * t9977;
    let t12765 = t1440 * t3675;
    let t12770 = 24.0 / 5.0 * t1325 * t12765 * t806 * t1392 * t494;
    let t12771 = t806 * t542;
    let t12775 = 12.0 / 5.0 * t519 * t12765 * t12771 * t1278;
    (t12757, t12758, t12759, t12760, t12761, t12762, t12763, t12764, t12770, t12775)
}
