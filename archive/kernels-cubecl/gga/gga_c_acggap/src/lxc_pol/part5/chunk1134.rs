//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1134/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1134<F: Float>(t406: F, t495: F, t1454: F, t322: F, t13287: F, t13293: F, t525: F, t13298: F, t176: F, t5730: F, t8401: F, t13299: F, t17173: F, t5605: F, t8790: F) -> (F, F, F, F, F) {
    let t20305 = t495 * t406;
    let t20311 = t1454 * t322;
    let t20314 = t13293 * t13287 * t525 * t20311;
    let t20323 = t13298 * t176 * t8401 * t5730;
    let t20328 = t17173 * t13299 * t8790 * t5605 * t322;
    (t20305, t20311, t20314, t20323, t20328)
}
