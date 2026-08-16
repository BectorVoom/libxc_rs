//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 555/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk555<F: Float>(t3228: F, t425: F, t431: F, t438: F, t1195: F, t997: F, t377: F, t996: F) -> (F, F, F, F, F) {
    let t3229 = t3228 * t425;
    let t3231 = t3228 * t431;
    let t3233 = t3228 * t438;
    let t3235 = t997 * t1195;
    let t3237 = t377 * t996;
    (t3229, t3231, t3233, t3235, t3237)
}
