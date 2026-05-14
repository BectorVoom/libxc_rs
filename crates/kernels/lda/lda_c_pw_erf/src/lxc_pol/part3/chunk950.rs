//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 950/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk950<F: Float>(t1472: F, t4930: F, t1403: F, t1466: F, t2065: F, t3667: F, t571: F, t10505: F, t799: F, t2178: F, t3745: F, t1339: F, t2176: F, t348: F, t519: F, t1486: F, t352: F, t4867: F) -> (F, F, F, F, F, F) {
    let t12582 = 12.0 / 5.0 * t1472 * t4930;
    let t12587 = 12.0 / 5.0 * t571 * t1466 * t3667 * t2065 * t1403;
    let t12589 = 8.0 / 15.0 * t10505 * t799;
    let t12591 = 16.0 / 15.0 * t3745 * t2178;
    let t12595 = 16.0 / 15.0 * t519 * t2176 * t1339 * t348;
    let t12599 = 8.0 / 9.0 * t571 * t4867 * t1486 * t352;
    (t12582, t12587, t12589, t12591, t12595, t12599)
}
