//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1025/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1025<F: Float>(t1184: F, t2152: F, t571: F, t573: F, t1446: F, t5339: F, t2177: F, t519: F, t521: F, t12064: F, t4523: F, t2137: F, t4073: F, t3445: F, t822: F, t2120: F, t3387: F) -> (F, F, F, F, F, F, F) {
    let t13515 = t571 * t1184 * t573 * t2152;
    let t13517 = t1446 * t5339;
    let t13523 = t519 * t1184 * t521 * t2177;
    let t13538 = t12064 * t4523;
    let t13540 = t4073 * t2137;
    let t13542 = t822 * t3445;
    let t13544 = t2120 * t3387;
    (t13515, t13517, t13523, t13538, t13540, t13542, t13544)
}
