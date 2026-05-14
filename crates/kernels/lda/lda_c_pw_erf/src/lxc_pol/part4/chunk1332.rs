//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1332/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1332<F: Float>(t18273: F, t18277: F, t18279: F, t18281: F, t18283: F, t18287: F, t18289: F, t18291: F, t18293: F, t18295: F, t18296: F, t18300: F, t18302: F, t18306: F, t18309: F, t18312: F, t18315: F) -> (F,) {
    let t19299 = t18273 + t18277 + t18279 - t18281 - t18283 + t18287 - t18289 - t18291 + t18293 - t18295 + t18296 + t18300 - t18302 - t18306 - t18309 - t18312 - t18315;
    (t19299,)
}
