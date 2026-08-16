//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1150/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1150<F: Float>(t21128: F, t21129: F, t21130: F, t21131: F, t21132: F, t21133: F, t21134: F, t21135: F, t21136: F, t21140: F, t21142: F, t21144: F, t21146: F, t21150: F) -> F {
    let t21151 = t21128 + t21129 + t21130 - t21131 - t21132 - t21133 + t21134 + t21135 - t21136 + t21140 + t21142 + t21144 - t21146 - t21150;
    t21151
}
