//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 374/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk374<F: Float>(t1049: F, t503: F, t1055: F, t1427: F, t345: F, t355: F, t495: F, t721: F, t1060: F, t1298: F, t346: F, t1048: F, t1050: F, t1054: F, t1063: F, t1076: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1474 = t1049 * t503;
    let t1476 = t1055 * t1427;
    let t1477 = t345 * t1476;
    let t1479 = t355 * t495;
    let t1480 = t1479 * t721;
    let t1481 = t1060 * t1480;
    let t1483 = t346 * t1298;
    let t1484 = t345 * t1483;
    let t1487 = t1048 + t1050 / F::new(3.0) - t1054 + t1474 / F::new(3.0) + t1477 / F::new(2.0) - t1481 / F::new(24.0) - t1484 / F::new(4.0) - t1063 / F::new(24.0) + t1076;
    (t1474, t1476, t1477, t1479, t1480, t1481, t1483, t1484, t1487)
}
