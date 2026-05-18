//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 769/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk769<F: Float>(t611: F, t9128: F, t3085: F, t3160: F, t608: F, t1013: F, t1924: F, t1016: F, t1019: F, t1386: F, t605: F, t1717: F, t8999: F) -> (F, F, F, F, F, F) {
    let t9129 = t611 * t9128;
    let t9130 = t9129 * t3085;
    let t9132 = t3160 * t608;
    let t9135 = t1013 * t1924;
    let t9138 = t1386 * t1016 * t1019;
    let t9140 = t3160 * t605;
    let t9142 = t8999 * t1717;
    (t9130, t9132, t9135, t9138, t9140, t9142)
}
