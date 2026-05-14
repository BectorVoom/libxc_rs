//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 761/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk761<F: Float>(t5305: F, t951: F, t1308: F, t571: F, t2026: F, t3859: F, t1325: F, t1469: F, t4763: F, t2065: F, t581: F, t549: F, t1466: F, t1318: F, t1401: F, t593: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5306 = t5305 * t951;
    let t5307 = t1308 * t5306;
    let t5309 = 8.0 / 45.0 * t571 * t5307;
    let t5310 = t3859 * t2026;
    let t5312 = 32.0 / 135.0 * t1325 * t5310;
    let t5314 = 8.0 / 15.0 * t4763 * t1469;
    let t5315 = t581 * t2065;
    let t5316 = t5315 * t549;
    let t5317 = t1466 * t5316;
    let t5319 = 8.0 / 15.0 * t1318 * t5317;
    let t5320 = t1401 * t2065;
    let t5321 = t5320 * t593;
    (t5306, t5307, t5309, t5310, t5312, t5314, t5316, t5317, t5319, t5321)
}
