//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 264/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk264<F: Float>(t425: F, t997: F, t431: F, t438: F, t150: F, t851: F) -> (F, F, F, F) {
    let t998 = t997 * t425;
    let t1000 = t997 * t431;
    let t1002 = t997 * t438;
    let t1004 = t851 * t150;
    (t998, t1000, t1002, t1004)
}
