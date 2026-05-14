//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 874/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk874<F: Float>(t5677: F, t684: F, t5681: F, t1738: F, t2306: F, t1729: F, t1880: F, t405: F, t6153: F, t5782: F, t140: F, t6126: F, t159: F, t1904: F, t285: F, t39: F) -> (F, F, F, F, F, F, F, F) {
    let t14468 = t684 * t5677;
    let t14469 = 0.11974234010254609 * t14468;
    let t14470 = t684 * t5681;
    let t14472 = t1738 * t2306;
    let t14473 = 0.15965645347006147 * t14472;
    let t14480 = t1729 * t1880;
    let t14485 = t405 * t6153;
    let t14488 = t1729 * t5782;
    let t14491 = t6126 * t140;
    let t14515 = t39 * t1904 * t159 * t285;
    (t14469, t14470, t14473, t14480, t14485, t14488, t14491, t14515)
}
