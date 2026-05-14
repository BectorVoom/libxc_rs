//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 810/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk810<F: Float>(t3210: F, t8879: F, t3309: F, t436: F, t2: F, t39: F, t411: F, t120: F, t3318: F, t119: F, t155: F, t3222: F, t1: F, t3296: F, t431: F, t1664: F, t473: F) -> (F, F, F, F, F, F, F, F) {
    let t8894 = t3210 * t8879;
    let t8896 = t3309 * t436;
    let t8898 = t2 * t39 * t411;
    let t8899 = t8896 * t8898;
    let t8901 = t3318 * t120;
    let t8902 = t8901 * t8898;
    let t8916 = t119 * t155 * t3222;
    let t8917 = t431 * t3296 * t1 * t8916;
    let t8920 = t119 * t473 * t1664;
    (t8894, t8896, t8899, t8901, t8902, t8916, t8917, t8920)
}
