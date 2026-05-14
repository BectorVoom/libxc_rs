//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1371/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1371<F: Float>(t2951: F, t4292: F, t4408: F, t4410: F, t4412: F, t4416: F, t4418: F, t5686: F, t5688: F, t5945: F, t6012: F, t6056: F, t7308: F, t8106: F, t8107: F, t8108: F, t8109: F, t8110: F, t8113: F, t8114: F) -> (F,) {
    let t19922 = 120.0 * t4408 + 0.03950357940513041 * t4410 + 7.017868076946245 * t4412 - 3.796345779698908 * t4416 + 0.03950357940513041 * t6012 + 4.107632884006667 * t4418 - 2.0 * t7308 - t8106 + t8107 - t8108 + t8109 - t8110 - 0.0003662311007350632 * t6056 - t8113 + t8114 + 7.017868076946245 * t2951 - 3.796345779698908 * t4292 - 2.0 * t5945 + 8.0 * t5686 - 32.0 * t5688;
    (t19922,)
}
