//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1069/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1069<F: Float>(t1729: F, t1880: F, t405: F, t6153: F, t5782: F, t140: F, t6126: F, t10832: F, t1872: F, t5673: F, t684: F, t2765: F, t5647: F, t5643: F, t159: F, t1904: F, t285: F, t39: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14480 = t1729 * t1880;
    let t14485 = t405 * t6153;
    let t14488 = t1729 * t5782;
    let t14491 = t6126 * t140;
    let t14500 = t10832 * t1872;
    let t14503 = t684 * t5673;
    let t14505 = t2765 * t5647;
    let t14508 = t2765 * t5643;
    let t14515 = t39 * t1904 * t159 * t285;
    (t14480, t14485, t14488, t14491, t14500, t14503, t14505, t14508, t14515)
}
