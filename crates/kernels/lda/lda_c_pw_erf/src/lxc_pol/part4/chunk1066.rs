//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1066/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1066<F: Float>(t2544: F, t3742: F, t10605: F, t2543: F, t571: F, t2171: F, t5360: F, t5397: F, t1403: F, t1466: F, t2466: F, t9237: F, t4738: F, t4882: F, t2153: F, t5334: F) -> (F, F, F, F, F, F, F) {
    let t15519 = 8.0 / 27.0 * t3742 * t2544;
    let t15521 = t571 * t10605 * t2543;
    let t15522 = 8.0 / 243.0 * t15521;
    let t15524 = 8.0 / 15.0 * t2171 * t5360;
    let t15525 = t2171 * t5397;
    let t15526 = 32.0 / 45.0 * t15525;
    let t15531 = 16.0 / 5.0 * t571 * t1466 * t9237 * t2466 * t1403;
    let t15533 = 16.0 / 15.0 * t4738 * t4882;
    let t15535 = 32.0 / 45.0 * t5334 * t2153;
    (t15519, t15522, t15524, t15526, t15531, t15533, t15535)
}
