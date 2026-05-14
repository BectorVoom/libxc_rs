//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 776/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk776<F: Float>(t1563: F, t34: F, t1820: F, t1823: F, t1826: F, t1829: F, t39: F, t406: F, t408: F, t4356: F, t4371: F, t462: F, t5524: F, t5527: F, t5536: F, t940: F, t945: F, t951: F, t954: F) -> (F,) {
    let t5539 = t1563 * t34;
    let t5548 = 4.0 / 27.0 * t5524 * t940 - 4.0 / 9.0 * t5527 * t4356 - t1820 * t945 / 9.0 + 2.0 / 3.0 * t406 * t462 - 2.0 * t1823 * t39 + 4.0 / 27.0 * t5536 * t951 + 4.0 / 9.0 * t5539 * t4371 - t1826 * t954 / 9.0 - 2.0 / 3.0 * t408 * t462 + 2.0 * t1829 * t39;
    (t5548,)
}
