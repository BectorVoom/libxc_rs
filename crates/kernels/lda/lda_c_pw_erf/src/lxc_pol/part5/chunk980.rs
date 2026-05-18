//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 980/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk980<F: Float>(t1729: F, t5782: F, t140: F, t6126: F, t159: F, t1904: F, t285: F, t39: F, t1125: F, t763: F, t133: F, t1844: F, t474: F) -> (F, F, F, F, F, F) {
    let t14488 = t1729 * t5782;
    let t14491 = t6126 * t140;
    let t14515 = t39 * t1904 * t159 * t285;
    let t14516 = F::new(0.004067943812504169) * t14515;
    let t14581 = t1125 * t763;
    let t14582 = t133 * t14581;
    let t14584 = t474 * t1844;
    (t14488, t14491, t14516, t14581, t14582, t14584)
}
