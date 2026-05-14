//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1020/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1020<F: Float>(t10427: F, t10429: F, t493: F, t9248: F, t10432: F, t10439: F, t3704: F, t4505: F, t34: F, t352: F, t593: F, t4522: F, t1287: F, t743: F, t3974: F, t5160: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t13765 = 16.0 / 135.0 * t10427;
    let t13766 = 8.0 / 15.0 * t10429;
    let t13767 = t493 * t9248;
    let t13768 = 16.0 / 15.0 * t13767;
    let t13769 = 8.0 / 15.0 * t10432;
    let t13770 = 8.0 / 45.0 * t10439;
    let t13771 = t4505 * t3704;
    let t13773 = t34 * t593 * t352;
    let t13776 = 16.0 / 9.0 * t13771 * t4522 * t13773;
    let t13777 = t743 * t1287;
    let t13778 = t13777 * t352;
    let t13781 = 16.0 / 15.0 * t3974 * t5160 * t13778;
    (t13765, t13766, t13768, t13769, t13770, t13771, t13773, t13776, t13777, t13778, t13781)
}
