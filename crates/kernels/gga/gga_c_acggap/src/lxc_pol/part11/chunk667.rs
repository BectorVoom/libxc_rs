//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 667/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk667<F: Float>(t438: F, t7605: F, t1205: F, t2001: F, t597: F, t980: F) -> (F, F, F, F) {
    let t7606 = t7605 * t438;
    let t7607 = 0.17149607247227894789e-2 * t7606;
    let t7608 = t2001 * t1205;
    let t7610 = t980 * t597;
    (t7606, t7607, t7608, t7610)
}
