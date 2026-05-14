//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 828/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk828<F: Float>(t3416: F, t3900: F, t1401: F, t1475: F, t3868: F, t571: F, t3893: F, t529: F, t3802: F, t3846: F, t519: F, t3412: F, t3859: F, t3482: F, t5237: F, t1325: F, t3398: F) -> (F, F, F, F, F, F, F, F) {
    let t9647 = t3416 * t3900;
    let t9678 = t1475 * t1401;
    let t9680 = t571 * t9678 * t3868;
    let t9700 = t3893 * t529;
    let t9711 = t519 * t3802 * t3846;
    let t9714 = t519 * t3859 * t3412;
    let t9718 = t519 * t5237 * t3482;
    let t9721 = t1325 * t5237 * t3398;
    (t9647, t9678, t9680, t9700, t9711, t9714, t9718, t9721)
}
