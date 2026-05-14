//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 866/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk866<F: Float>(t3631: F, t783: F, t34: F, t3615: F, t109: F, t1282: F, t2247: F, t5875: F, t2249: F, t370: F, t409: F, t5858: F, t5866: F, t5870: F, t11211: F, t11222: F, t11225: F, t11229: F, t11236: F, t11289: F, t1234: F, t2209: F, t3588: F, t5874: F, t769: F, t8428: F) -> (F, F) {
    let t11465 = t783 * t3631;
    let t11470 = t34 * t3615;
    let t11475 = t109 * t1282;
    let t11477 = t2247 * t11475 * t5875;
    let t11485 = t2247 * t409 * t370 * t2249;
    let t11488 = t2247 * t5858 * t5866;
    let t11491 = t2247 * t5858 * t5870;
    let t11493 = t11211 - t11222 + t11225 + t11229 - t11236 + t8428 - t11289 + 103.4553 * t2247 * t11470 * t769 * t3588 + 20.69106 * t11477 - 62.07318 * t2247 * t5874 * t2209 * t1234 + 6.89702 * t11485 - 10.34553 * t11488 - 5.172765 * t11491;
    (t11465, t11493)
}
