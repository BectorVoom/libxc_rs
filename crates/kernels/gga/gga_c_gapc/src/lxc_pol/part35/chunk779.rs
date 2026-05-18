//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 779/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk779<F: Float>(t1013: F, t1924: F, t1016: F, t1019: F, t1386: F, t3160: F, t605: F, t1717: F, t8999: F, t633: F, t8769: F, t1700: F) -> (F, F, F, F, F) {
    let t9135 = t1013 * t1924;
    let t9138 = t1386 * t1016 * t1019;
    let t9140 = t3160 * t605;
    let t9142 = t8999 * t1717;
    let t9144 = t633 * t8769;
    let t9145 = t9144 * t1700;
    (t9135, t9138, t9140, t9142, t9145)
}
