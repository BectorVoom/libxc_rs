//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1136/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1136<F: Float>(t1325: F, t3859: F, t6264: F, t1251: F, t2328: F, t1326: F, t940: F, t2171: F, t5421: F, t5426: F, t12765: F, t12771: F, t2098: F, t519: F, t494: F, t5289: F, t806: F) -> (F, F, F, F, F, F) {
    let t16702 = t1325 * t3859 * t6264;
    let t16703 = 32.0 / 135.0 * t16702;
    let t16704 = t2328 * t1251;
    let t16708 = 16.0 / 45.0 * t1325 * t1326 * t16704 * t940;
    let t16709 = t2171 * t5421;
    let t16710 = 32.0 / 135.0 * t16709;
    let t16712 = 16.0 / 45.0 * t2171 * t5426;
    let t16716 = 16.0 / 5.0 * t519 * t12765 * t12771 * t2098;
    let t16721 = 32.0 / 15.0 * t1325 * t5289 * t806 * t2098 * t494;
    (t16703, t16708, t16710, t16712, t16716, t16721)
}
