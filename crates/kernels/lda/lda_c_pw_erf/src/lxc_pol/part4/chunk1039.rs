//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1039/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1039<F: Float>(t1772: F, t3007: F, t2765: F, t411: F, t4429: F, t5677: F, t684: F, t5681: F, t1738: F, t2306: F, t405: F, t6153: F, t1729: F, t5782: F, t140: F, t6126: F) -> (F, F, F, F, F, F, F, F) {
    let t14449 = t1772 * t3007;
    let t14465 = t2765 * t4429 * t411;
    let t14468 = t684 * t5677;
    let t14470 = t684 * t5681;
    let t14472 = t1738 * t2306;
    let t14485 = t405 * t6153;
    let t14488 = t1729 * t5782;
    let t14491 = t6126 * t140;
    (t14449, t14465, t14468, t14470, t14472, t14485, t14488, t14491)
}
