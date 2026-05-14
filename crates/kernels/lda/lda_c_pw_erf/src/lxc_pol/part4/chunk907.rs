//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 907/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk907<F: Float>(t4215: F, t4217: F, t4220: F, t4227: F, t4235: F, t5236: F, t5240: F, t5284: F, t5304: F, t6862: F, t6864: F, t6869: F, t6871: F, t6873: F, t6877: F, t6879: F) -> (F,) {
    let t7275 = t6862 + t6864 + t6869 - t6871 - t6873 - t6877 + t6879 + t4215 + t4217 + 8.0 / 3.0 * t4220 + 4.0 / 3.0 * t4227 + t4235 - t5236 + t5240 + t5284 - t5304;
    (t7275,)
}
