//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1328/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1328<F: Float>(t18113: F, t18114: F, t18119: F, t18121: F, t18123: F, t18125: F, t18127: F, t18130: F, t18132: F, t18137: F, t18142: F, t18143: F, t18144: F, t18145: F, t18149: F, t18153: F, t18156: F) -> (F,) {
    let t19289 = -t18113 - t18114 + t18119 + t18121 - t18123 - t18125 - t18127 + t18130 + t18132 + t18137 + t18142 + t18143 - t18144 + t18145 + t18149 + t18153 + t18156;
    (t19289,)
}
