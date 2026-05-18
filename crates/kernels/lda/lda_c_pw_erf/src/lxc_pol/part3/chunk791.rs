//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 791/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk791<F: Float>(t5302: F, t571: F, t1333: F, t833: F, t951: F, t1308: F, t2026: F, t3859: F, t1325: F, t1469: F, t4763: F, t2065: F, t581: F) -> (F, F, F, F, F, F, F, F) {
    let t5304 = F::new(16.0) / F::new(135.0) * t571 * t5302;
    let t5305 = t833 * t1333;
    let t5306 = t5305 * t951;
    let t5307 = t1308 * t5306;
    let t5309 = F::new(8.0) / F::new(45.0) * t571 * t5307;
    let t5310 = t3859 * t2026;
    let t5312 = F::new(32.0) / F::new(135.0) * t1325 * t5310;
    let t5314 = F::new(8.0) / F::new(15.0) * t4763 * t1469;
    let t5315 = t581 * t2065;
    (t5304, t5306, t5307, t5309, t5310, t5312, t5314, t5315)
}
