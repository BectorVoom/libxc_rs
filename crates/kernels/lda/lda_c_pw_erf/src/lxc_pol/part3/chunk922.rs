//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 922/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk922<F: Float>(t1298: F, t3550: F, t1301: F, t1518: F, t493: F, t2070: F, t543: F, t185: F, t3553: F, t511: F, t1294: F, t1498: F) -> (F, F, F, F, F, F) {
    let t9941 = t1298 * t3550;
    let t9944 = t493 * t1518 * t1301;
    let t9946 = t2070 * t543;
    let t9947 = t185 * t9946;
    let t9949 = t511 * t3553;
    let t9953 = t1498 * t1294;
    (t9941, t9944, t9946, t9947, t9949, t9953)
}
