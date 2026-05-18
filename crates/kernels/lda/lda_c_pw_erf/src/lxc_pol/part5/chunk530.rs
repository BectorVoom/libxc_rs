//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 530/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk530<F: Float>(t2530: F, t2534: F, t2538: F, t2542: F, t2546: F, t2548: F, t2552: F, t2556: F, t2560: F, t2564: F, t2568: F, t2569: F) -> F {
    let t2665 = -t2530 - t2534 + t2538 + t2542 + t2546 + t2548 + t2552 + t2556 - t2560 - t2564 - t2568 + t2569;
    t2665
}
