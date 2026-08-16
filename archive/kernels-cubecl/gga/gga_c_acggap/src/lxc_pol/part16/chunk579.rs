//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 579/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk579<F: Float>(t368: F, t398: F, t5087: F, t384: F, t3476: F, t527: F, t513: F, t864: F, t1095: F, t1036: F, t1032: F, t1434: F) -> (F, F, F, F, F, F, F) {
    let t5089 = t398 * t368 * t5087;
    let t5090 = t384 * t5089;
    let t5092 = t3476 * t527;
    let t5099 = t513 * t864;
    let t5101 = t398 * t1095 * t5099;
    let t5102 = t1036 * t5101;
    let t5104 = t1032 * t1434;
    (t5089, t5090, t5092, t5099, t5101, t5102, t5104)
}
