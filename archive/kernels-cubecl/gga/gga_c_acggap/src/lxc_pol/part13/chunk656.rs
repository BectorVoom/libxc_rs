//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 656/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk656<F: Float>(t513: F, t922: F, t1095: F, t1426: F, t1175: F, t360: F, t1181: F, t1532: F, t372: F, t1165: F, t1552: F, t3196: F) -> (F, F, F, F, F, F, F) {
    let t5265 = t513 * t922;
    let t5267 = t1426 * t1095 * t5265;
    let t5270 = t1175 * t360;
    let t5272 = t1181 * t1532 * t5270;
    let t5275 = t1175 * t372;
    let t5277 = t1165 * t1552 * t5275;
    let t5281 = t1165 * t1532 * t3196;
    (t5265, t5267, t5270, t5272, t5275, t5277, t5281)
}
