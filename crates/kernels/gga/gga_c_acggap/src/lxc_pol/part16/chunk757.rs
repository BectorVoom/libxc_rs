//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 757/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk757<F: Float>(t1089: F, t5011: F, t9552: F, t598: F, t1817: F, t7733: F, t2288: F, t4643: F, t137: F, t1795: F) -> (F, F, F, F, F, F) {
    let t9554 = t1089 * t5011 * t9552;
    let t9555 = t598 * t9554;
    let t9557 = t7733 * t1817;
    let t9560 = t1089 * t4643 * t2288;
    let t9561 = t598 * t9560;
    let t9563 = t137 * t1795;
    (t9554, t9555, t9557, t9560, t9561, t9563)
}
