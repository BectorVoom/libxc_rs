//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1050/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1050<F: Float>(t12299: F, t1443: F, t1287: F, t1318: F, t1466: F, t5315: F, t3899: F, t5321: F, t571: F, t3663: F, t822: F, t1294: F, t1960: F) -> (F, F, F, F, F) {
    let t12301 = F::new(8.0) / F::new(5.0) * t12299 * t1443;
    let t12305 = F::new(4.0) / F::new(5.0) * t1318 * t1466 * t5315 * t1287;
    let t12307 = t571 * t3899 * t5321;
    let t12308 = F::new(16.0) / F::new(15.0) * t12307;
    let t12309 = t822 * t3663;
    let t12310 = F::new(4.0) / F::new(45.0) * t12309;
    let t12311 = t1960 * t1294;
    (t12301, t12305, t12308, t12310, t12311)
}
