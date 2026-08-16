//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 771/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk771<F: Float>(t7951: F, t8019: F, t105: F, t469: F, t3952: F, t811: F, t1268: F, t814: F, t103: F, t566: F, t95: F, t1427: F, t7278: F) -> (F, F, F, F, F, F, F) {
    let t8020 = t7951 + t8019;
    let t8021 = t105 * t8020;
    let t8022 = t8021 * t469;
    let t8027 = t3952 * t811;
    let t8031 = t814 * t1268;
    let t8372 = t566 * t95 * t103;
    let t8373 = t7278 * t1427;
    (t8020, t8021, t8022, t8027, t8031, t8372, t8373)
}
