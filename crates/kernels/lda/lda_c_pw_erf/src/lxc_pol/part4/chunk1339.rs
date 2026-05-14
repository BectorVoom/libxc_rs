//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1339/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1339<F: Float>(t18565: F, t18567: F, t18569: F, t18573: F, t18576: F, t18578: F, t18583: F, t18585: F, t18589: F, t18591: F, t18594: F, t18597: F, t18600: F, t18602: F, t18607: F, t18610: F, t18612: F) -> (F,) {
    let t19326 = t18565 + t18567 + t18569 - t18573 + t18576 - t18578 + t18583 - t18585 + t18589 + t18591 + t18594 - t18597 - t18600 + t18602 - t18607 + t18610 + t18612;
    (t19326,)
}
