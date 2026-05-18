//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 902/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk902<F: Float>(t13502: F, t425: F, t431: F, t438: F, t1005: F, t3786: F, t1032: F, t3266: F, t384: F, t386: F, t991: F, t1103: F, t3770: F) -> (F, F, F, F, F, F, F) {
    let t13503 = t13502 * t425;
    let t13505 = t13502 * t431;
    let t13507 = t13502 * t438;
    let t13509 = t1005 * t3786;
    let t13517 = t1032 * t3786;
    let t13521 = t384 * t386 * t3266 * t991;
    let t13532 = t3770 * t1103;
    (t13503, t13505, t13507, t13509, t13517, t13521, t13532)
}
