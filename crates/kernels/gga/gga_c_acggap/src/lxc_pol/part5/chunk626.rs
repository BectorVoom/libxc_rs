//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 626/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk626<F: Float>(t1101: F, t1165: F, t530: F, t4282: F, t1470: F, t3409: F, t1410: F, t174: F) -> (F, F, F, F) {
    let t4284 = t1165 * t530 * t1101;
    let t4285 = t4282 * t4284;
    let t4288 = 0.40015750243531754508e-2 * t3409 * t1470;
    let t4289 = t174 * t1410;
    (t4284, t4285, t4288, t4289)
}
