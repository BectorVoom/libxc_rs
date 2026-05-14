//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 726/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk726<F: Float>(t1089: F, t1459: F, t8484: F, t598: F, t355: F, t513: F, t7458: F, t1980: F, t1988: F, t2294: F, t2288: F, t3201: F, t1083: F, t2299: F, t1530: F, t7646: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8486 = t1089 * t1459 * t8484;
    let t8487 = t598 * t8486;
    let t8489 = t355 * t513;
    let t8491 = t7458 * t1459 * t8489;
    let t8492 = t1980 * t8491;
    let t8494 = t1988 * t2294;
    let t8497 = t1089 * t3201 * t2288;
    let t8498 = t598 * t8497;
    let t8502 = t1089 * t1083 * t8484;
    let t8503 = t598 * t8502;
    let t8505 = t1083 * t8489;
    let t8506 = t7458 * t8505;
    let t8507 = t1980 * t8506;
    let t8509 = t1988 * t2299;
    let t8511 = t1530 * t7646;
    (t8486, t8487, t8489, t8491, t8492, t8494, t8497, t8498, t8502, t8503, t8505, t8506, t8507, t8509, t8511)
}
