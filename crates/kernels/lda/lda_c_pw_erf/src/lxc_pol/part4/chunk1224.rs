//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1224/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1224<F: Float>(t4738: F, t5382: F, t595: F, t6671: F, t1280: F, t2443: F, t18130: F, t18132: F, t18137: F, t18142: F, t18143: F, t18144: F, t18145: F, t18149: F, t18153: F, t18156: F, t18159: F, t18161: F, t18164: F, t18168: F) -> (F, F, F, F) {
    let t18170 = 8.0 / 15.0 * t4738 * t5382;
    let t18172 = 4.0 / 15.0 * t6671 * t595;
    let t18174 = 2.0 / 15.0 * t2443 * t1280;
    let t18175 = t18130 + t18132 + t18137 + t18142 + t18143 - t18144 + t18145 + t18149 + t18153 + t18156 - t18159 - t18161 - t18164 - t18168 - t18170 - t18172 - t18174;
    (t18170, t18172, t18174, t18175)
}
