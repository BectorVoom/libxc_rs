//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 670/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk670<F: Float>(t4517: F, t4522: F, t4506: F, t3443: F, t3446: F, t3458: F, t3551: F, t3554: F, t3557: F, t3570: F, t3577: F, t3661: F, t3664: F, t1: F, t1904: F, t3: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4523 = t4522 * t4517;
    let t4525 = 8.0 / 27.0 * t4506 * t4523;
    let t4526 = 16.0 / 45.0 * t3443;
    let t4527 = 8.0 / 45.0 * t3446;
    let t4528 = 8.0 / 45.0 * t3458;
    let t4529 = 16.0 / 135.0 * t3551;
    let t4530 = 8.0 / 135.0 * t3554;
    let t4531 = 4.0 / 45.0 * t3557;
    let t4532 = 8.0 / 45.0 * t3570;
    let t4533 = 4.0 / 45.0 * t3577;
    let t4534 = 16.0 / 135.0 * t3661;
    let t4535 = 8.0 / 135.0 * t3664;
    let t4537 = t1904 * t1 * t3;
    (t4523, t4525, t4526, t4527, t4528, t4529, t4530, t4531, t4532, t4533, t4534, t4535, t4537)
}
