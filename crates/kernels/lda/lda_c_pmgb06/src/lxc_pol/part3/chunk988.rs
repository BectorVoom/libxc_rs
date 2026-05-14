//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 988/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk988<F: Float>(t13432: F, t1420: F, t4772: F, t1416: F, t493: F, t5312: F, t432: F, t4836: F, t13327: F, t13328: F, t13421: F, t13423: F, t13425: F, t13427: F, t13429: F, t13431: F) -> (F, F, F, F, F) {
    let t13433 = 2.0 / 15.0 * t13432;
    let t13435 = 2.0 / 15.0 * t1420 * t4772;
    let t13438 = 2.0 / 15.0 * t493 * t5312 * t1416;
    let t13439 = t432 * t4836;
    let t13440 = t13439 / 45.0;
    let t13441 = t13327 - t13328 + t13421 + t13423 + t13425 + t13427 - t13429 - t13431 + t13433 - t13435 - t13438 + t13440;
    (t13433, t13435, t13438, t13440, t13441)
}
