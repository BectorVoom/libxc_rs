//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1272/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1272<F: Float>(t15441: F, t15443: F, t15444: F, t15445: F, t15446: F, t15447: F, t15448: F, t15450: F, t15452: F, t15454: F, t15456: F, t15458: F, t8277: F, t8285: F, t8290: F, t8296: F, t8300: F, t8356: F) -> (F,) {
    let t18960 = -t8277 - t15441 + t15443 + t8285 + t15444 + t8290 + t15445 - t8296 - t15446 - t15447 - t15448 + t8300 + t15450 + t15452 - t8356 - t15454 + t15456 + t15458;
    (t18960,)
}
