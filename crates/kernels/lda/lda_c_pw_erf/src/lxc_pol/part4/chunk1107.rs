//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1107/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1107<F: Float>(t2824: F, t35: F, t108: F, t209: F, t220: F, t266: F, t3806: F, t519: F, t6426: F, t945: F, t1245: F, t2471: F, t940: F, t9504: F, t12781: F, t1325: F, t6432: F) -> (F, F, F, F, F) {
    let t16143 = t35 * t2824;
    let t16144 = t209 * t108;
    let t16148 = 8.0 / 15.0 * t16143 * t16144 * t266 * t220;
    let t16152 = 8.0 / 45.0 * t519 * t3806 * t6426 * t945;
    let t16153 = t2471 * t1245;
    let t16157 = 8.0 / 27.0 * t519 * t9504 * t16153 * t940;
    let t16159 = t1325 * t12781 * t6432;
    (t16143, t16148, t16152, t16157, t16159)
}
