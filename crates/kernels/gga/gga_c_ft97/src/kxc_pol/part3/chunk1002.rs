//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 1002/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk1002<F: Float>(t18497: F, t4140: F, t4139: F, t312: F, t5225: F, t684: F, t10492: F, t15370: F, t4176: F, t15369: F, t4635: F, t875: F) -> (F, F, F, F) {
    let t19513 = t4140 * t18497;
    let t19514 = t4139 * t19513;
    let t19517 = t312 * t5225;
    let t19518 = t19517 * t684;
    let t19519 = t10492 * t19518;
    let t19522 = t15370 * t4176;
    let t19523 = t15369 * t19522;
    let t19526 = t4635 * t875;
    (t19514, t19519, t19523, t19526)
}
