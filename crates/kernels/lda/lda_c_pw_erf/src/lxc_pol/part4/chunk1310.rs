//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1310/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1310<F: Float>(t17174: F, t17175: F, t17176: F, t17179: F, t17180: F, t17181: F, t17182: F, t17183: F, t17184: F, t17185: F, t17186: F, t17187: F, t17355: F, t17357: F, t17359: F, t17361: F, t17362: F) -> (F,) {
    let t19241 = -t17174 + t17175 - t17176 + t17179 - t17180 - t17181 + t17182 + t17183 - t17184 - t17185 - t17186 + t17187 - t17355 - t17357 - t17359 - t17361 + t17362;
    (t19241,)
}
