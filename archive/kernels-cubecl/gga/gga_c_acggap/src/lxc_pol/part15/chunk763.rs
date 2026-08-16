//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 763/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk763<F: Float>(t598: F, t8536: F, t1298: F, t137: F, t1426: F, t368: F, t1479: F, t7476: F, t1980: F, t1095: F, t1988: F, t2304: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8537 = t598 * t8536;
    let t8539 = t137 * t1298;
    let t8541 = t1426 * t368 * t8539;
    let t8542 = t598 * t8541;
    let t8544 = t368 * t1479;
    let t8545 = t7476 * t8544;
    let t8546 = t1980 * t8545;
    let t8549 = t1426 * t1095 * t8539;
    let t8550 = t598 * t8549;
    let t8555 = t7476 * t1095 * t1479;
    let t8556 = t1980 * t8555;
    let t8558 = t1988 * t2304;
    (t8537, t8539, t8541, t8542, t8544, t8545, t8546, t8549, t8550, t8555, t8556, t8558)
}
