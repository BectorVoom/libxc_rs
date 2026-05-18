//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 686/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk686<F: Float>(t1165: F, t604: F, t945: F, t7413: F, t955: F, t2068: F, t599: F, t1181: F, t2067: F, t3360: F) -> (F, F, F, F, F, F, F, F) {
    let t7415 = t1165 * t604 * t945;
    let t7416 = t7413 * t7415;
    let t7419 = t1165 * t604 * t955;
    let t7420 = t2068 * t7419;
    let t7422 = t599 * t955;
    let t7423 = t1181 * t7422;
    let t7424 = t2068 * t7423;
    let t7426 = t3360 * t2067;
    (t7415, t7416, t7419, t7420, t7422, t7423, t7424, t7426)
}
