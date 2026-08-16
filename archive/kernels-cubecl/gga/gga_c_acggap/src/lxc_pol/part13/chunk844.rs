//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 844/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk844<F: Float>(t157: F, t20432: F, t435: F, t507: F, t495: F, t930: F, t1298: F, t407: F, t5746: F, t943: F, t955: F, t1188: F, t1410: F) -> (F, F, F, F, F, F, F) {
    let t20433 = t20432 * t157;
    let t20559 = t507 * t435;
    let t20590 = t930 * t495;
    let t20595 = t407 * t1298;
    let t20775 = t5746 * t943;
    let t20817 = t955 * t495;
    let t20935 = t1188 * t1410;
    (t20433, t20559, t20590, t20595, t20775, t20817, t20935)
}
