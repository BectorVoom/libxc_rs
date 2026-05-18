//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1149/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1149<F: Float>(t21137: F, t519: F, t5256: F, t4804: F, t7702: F, t3794: F, t1472: F, t7710: F, t1308: F, t571: F, t6665: F, t833: F) -> (F, F, F, F, F) {
    let t21140 = F::new(8.0) / F::new(9.0) * t519 * t5256 * t21137;
    let t21142 = F::new(8.0) / F::new(9.0) * t4804 * t7702;
    let t21144 = F::new(8.0) / F::new(9.0) * t3794 * t7702;
    let t21146 = F::new(4.0) / F::new(15.0) * t1472 * t7710;
    let t21150 = F::new(4.0) / F::new(15.0) * t571 * t1308 * t6665 * t833;
    (t21140, t21142, t21144, t21146, t21150)
}
