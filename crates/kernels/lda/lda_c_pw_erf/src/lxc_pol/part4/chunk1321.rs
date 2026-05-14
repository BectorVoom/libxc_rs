//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1321/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1321<F: Float>(t17743: F, t17745: F, t17747: F, t17751: F, t17754: F, t17756: F, t17758: F, t17763: F, t17767: F, t17769: F, t17771: F, t17772: F, t17773: F, t17776: F, t17778: F, t17780: F, t17782: F) -> (F,) {
    let t19270 = t17743 + t17745 + t17747 - t17751 + t17754 - t17756 - t17758 - t17763 - t17767 + t17769 - t17771 + t17772 + t17773 + t17776 - t17778 - t17780 - t17782;
    (t19270,)
}
