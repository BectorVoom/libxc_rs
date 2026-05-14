//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 687/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk687<F: Float>(t1175: F, t372: F, t1165: F, t1552: F, t1532: F, t3196: F, t1539: F, t301: F) -> (F, F, F, F) {
    let t5275 = t1175 * t372;
    let t5277 = t1165 * t1552 * t5275;
    let t5281 = t1165 * t1532 * t3196;
    let t5284 = t1539 * t301;
    (t5275, t5277, t5281, t5284)
}
