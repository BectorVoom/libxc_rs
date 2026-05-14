//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 385/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk385<F: Float>(t1450: F, t519: F, t523: F, t945: F, t522: F, t187: F, t504: F) -> (F, F, F, F, F, F) {
    let t1451 = t519 * t1450;
    let t1452 = 16.0 / 135.0 * t1451;
    let t1453 = t523 * t945;
    let t1454 = t522 * t1453;
    let t1456 = 4.0 / 45.0 * t519 * t1454;
    let t1458 = 1.0 / t187 / t504;
    (t1451, t1452, t1453, t1454, t1456, t1458)
}
