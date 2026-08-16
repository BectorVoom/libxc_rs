//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1061/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1061<F: Float>(t15306: F, t15307: F, t2952: F, t4293: F, t4416: F, t4418: F, t6012: F, t6056: F, t7308: F, t7350: F, t8077: F, t8106: F, t8107: F, t8108: F, t8109: F, t8110: F, t8113: F, t8114: F, t8118: F) -> F {
    let t19967 = -F::cast_from(5.694518669548362_f64) * t4416 + F::cast_from(0.05925536910769562_f64) * t6012 + F::cast_from(6.16144932601_f64) * t4418 - F::cast_from(3.0_f64) * t7308 - t8106 + t8107 - t8108 + t8109 - t8110 - F::cast_from(0.0010986933022051897_f64) * t6056 - t8077 - t8113 + t8114 + t2952 - t4293 + t15306 - t15307 - t7350 - t8118;
    t19967
}
