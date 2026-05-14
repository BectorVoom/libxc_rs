//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 990/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk990<F: Float>(t9246: F, t15743: F, t15750: F, t2031: F, t6198: F, t1987: F, t1992: F, t10056: F, t352: F, t7365: F, t4776: F, t571: F, t34: F, t6378: F, t4868: F, t2100: F, t2443: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t20897 = 16.0 / 405.0 * t9246;
    let t20898 = 32.0 / 45.0 * t15743;
    let t20899 = 16.0 / 27.0 * t15750;
    let t20901 = 4.0 / 15.0 * t6198 * t2031;
    let t20903 = 8.0 / 15.0 * t6198 * t1987;
    let t20905 = 4.0 / 9.0 * t6198 * t1992;
    let t20907 = t10056 * t7365 * t352;
    let t20910 = 128.0 / 27.0 * t571 * t4776 * t20907;
    let t20911 = t6378 * t34;
    let t20914 = 16.0 / 3.0 * t571 * t4868 * t20911;
    let t20916 = 2.0 / 5.0 * t2443 * t2100;
    (t20897, t20898, t20899, t20901, t20903, t20905, t20907, t20910, t20911, t20914, t20916)
}
