//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 598/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk598<F: Float>(t1198: F, t336: F, t513: F, t1131: F, t157: F, t1165: F, t1552: F, t1532: F, t930: F, t530: F, t1162: F, t4198: F) -> (F, F, F, F, F) {
    let t4434 = t336 * t1198 * t513;
    let t4437 = t157 * t1131;
    let t4439 = t1165 * t1552 * t4437;
    let t4443 = t1165 * t1532 * t930;
    let t4447 = t1165 * t530 * t930;
    let t4450 = t4198 * t1162;
    (t4434, t4439, t4443, t4447, t4450)
}
