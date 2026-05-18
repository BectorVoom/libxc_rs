//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1314/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1314<F: Float>(t21169: F, t21173: F, t21175: F, t21179: F, t21183: F, t21185: F, t21189: F, t21193: F, t21195: F, t21199: F, t21204: F, t21206: F, t21210: F) -> F {
    let t23224 = -t21169 + t21173 - t21175 - t21179 - t21183 - t21185 - t21189 - t21193 + t21195 + t21199 + t21204 + t21206 + t21210;
    t23224
}
