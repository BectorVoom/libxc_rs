//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 931/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk931<F: Float>(t14345: F, t947: F, t1037: F, t1181: F, t3169: F, t3451: F, t1160: F, t3402: F, t1172: F, t3088: F, t1165: F, t3196: F) -> (F, F, F, F, F) {
    let t14346 = t14345 * t947;
    let t14357 = t3451 * t1181 * t1037 * t3169;
    let t14368 = t1160 * t3402;
    let t14373 = t3088 * t1172;
    let t14376 = t14373 * t1165 * t1037 * t3196;
    (t14346, t14357, t14368, t14373, t14376)
}
