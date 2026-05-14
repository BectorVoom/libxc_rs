//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 898/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk898<F: Float>(t3439: F, t4539: F, t4540: F, t6289: F, t6291: F, t6294: F, t6296: F, t6299: F, t6300: F, t6302: F, t6305: F, t6308: F, t6309: F, t6310: F, t6312: F, t6313: F) -> (F,) {
    let t7242 = -t6289 + t6291 + t6294 - t6296 - t6299 - t6300 + t3439 + t6302 + t6305 + t6308 - t6309 + t6310 - t6312 + t6313 + t4539 + 0.4328416544945937 * t4540;
    (t7242,)
}
