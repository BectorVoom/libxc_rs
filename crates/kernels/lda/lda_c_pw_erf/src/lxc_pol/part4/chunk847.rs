//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 847/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk847<F: Float>(t542: F, t6431: F, t5289: F, t1325: F, t2031: F, t2171: F, t1987: F, t1992: F, t1971: F, t34: F, t4829: F, t519: F, t1948: F, t4758: F, t571: F, t2393: F, t4804: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6432 = t6431 * t542;
    let t6433 = t5289 * t6432;
    let t6435 = 16.0 / 15.0 * t1325 * t6433;
    let t6437 = 8.0 / 45.0 * t2171 * t2031;
    let t6439 = 16.0 / 45.0 * t2171 * t1987;
    let t6441 = 8.0 / 27.0 * t2171 * t1992;
    let t6442 = t1971 * t34;
    let t6443 = t4829 * t6442;
    let t6445 = 32.0 / 45.0 * t519 * t6443;
    let t6446 = t1948 * t34;
    let t6447 = t4758 * t6446;
    let t6449 = 32.0 / 45.0 * t571 * t6447;
    let t6451 = 16.0 / 45.0 * t4804 * t2393;
    (t6432, t6433, t6435, t6437, t6439, t6441, t6442, t6443, t6445, t6446, t6447, t6449, t6451)
}
