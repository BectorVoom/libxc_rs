//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 670/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk670<F: Float>(t4758: F, t6446: F, t571: F, t2393: F, t4804: F, t3794: F, t2005: F, t34: F, t4829: F, t1325: F, t1446: F, t2397: F, t2098: F, t789: F, t1313: F, t519: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t6447 = t4758 * t6446;
    let t6449 = 32.0 / 45.0 * t571 * t6447;
    let t6451 = 16.0 / 45.0 * t4804 * t2393;
    let t6453 = 16.0 / 45.0 * t3794 * t2393;
    let t6454 = t2005 * t34;
    let t6455 = t4829 * t6454;
    let t6457 = 32.0 / 45.0 * t1325 * t6455;
    let t6459 = 8.0 / 45.0 * t1446 * t2397;
    let t6460 = t789 * t2098;
    let t6461 = t1313 * t6460;
    let t6463 = 8.0 / 45.0 * t519 * t6461;
    (t6447, t6449, t6451, t6453, t6454, t6455, t6457, t6459, t6460, t6461, t6463)
}
