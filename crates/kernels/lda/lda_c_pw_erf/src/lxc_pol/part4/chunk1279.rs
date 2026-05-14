//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1279/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1279<F: Float>(t1063: F, t14616: F, t14631: F, t15355: F, t15381: F, t1558: F, t1563: F, t1820: F, t1826: F, t2325: F, t2329: F, t2334: F, t2337: F, t2849: F, t3234: F, t3243: F, t348: F, t35: F, t352: F, t462: F, t5524: F, t5536: F, t5992: F, t6005: F, t6101: F, t6106: F, t6111: F, t6116: F, t8949: F, t8962: F, t940: F, t945: F, t951: F, t954: F) -> (F,) {
    let t19097 = -28.0 / 81.0 * t8949 * t2325 * t940 + 32.0 / 27.0 * t5524 * t15355 + 4.0 / 27.0 * t6101 * t945 - 8.0 / 9.0 * t1558 * t35 * t1063 - 4.0 / 9.0 * t1820 * t462 + 4.0 / 3.0 * t1820 * t2849 + 4.0 / 27.0 * t3234 * t2329 * t940 - 2.0 / 9.0 * t1558 * t5992 * t348 - t6106 * t945 / 9.0 + t14616 - 28.0 / 81.0 * t8962 * t2334 * t951 - 32.0 / 27.0 * t5536 * t15381 + 4.0 / 27.0 * t6111 * t954 - 8.0 / 9.0 * t1563 * t35 * t1063 + 4.0 / 9.0 * t1826 * t462 - 4.0 / 3.0 * t1826 * t2849 + 4.0 / 27.0 * t3243 * t2337 * t951 - 2.0 / 9.0 * t1563 * t6005 * t352 - t6116 * t954 / 9.0 - t14631;
    (t19097,)
}
