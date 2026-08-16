//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1190/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1190<F: Float>(t21564: F, t21568: F, t21570: F, t21571: F, t21573: F, t21575: F, t21576: F, t21581: F, t21582: F, t21587: F, t21591: F, t21596: F, t21601: F) -> F {
    let t21602 = -t21564 + t21568 + t21570 - t21571 + t21573 + t21575 + t21576 - t21581 - t21582 + t21587 - t21591 + t21596 + t21601;
    t21602
}
