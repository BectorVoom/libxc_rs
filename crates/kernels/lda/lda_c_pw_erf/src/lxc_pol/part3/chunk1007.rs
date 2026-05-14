//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1007/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1007<F: Float>(t1318: F, t2157: F, t9432: F, t3732: F, t4738: F, t1472: F, t5342: F, t1184: F, t2152: F, t571: F, t573: F, t1446: F, t5339: F, t13486: F, t13489: F, t13491: F, t13494: F, t13496: F, t13498: F, t13500: F, t13505: F) -> (F, F, F, F, F, F) {
    let t13507 = t1318 * t9432 * t2157;
    let t13508 = 8.0 / 45.0 * t13507;
    let t13510 = 8.0 / 5.0 * t4738 * t3732;
    let t13511 = t1472 * t5342;
    let t13512 = 8.0 / 135.0 * t13511;
    let t13515 = t571 * t1184 * t573 * t2152;
    let t13516 = 128.0 / 135.0 * t13515;
    let t13517 = t1446 * t5339;
    let t13518 = 8.0 / 135.0 * t13517;
    let t13519 = -t13486 + t13489 - t13491 - t13494 - t13496 + t13498 + t13500 + t13505 + t13508 + t13510 - t13512 - t13516 - t13518;
    (t13508, t13510, t13512, t13516, t13518, t13519)
}
