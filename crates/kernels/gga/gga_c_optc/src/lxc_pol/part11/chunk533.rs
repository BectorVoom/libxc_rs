//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 533/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk533<F: Float>(t1146: F, t1523: F, t106: F, t454: F, t1550: F, t3169: F, t3183: F, t446: F, t2667: F) -> (F, F, F, F, F) {
    let t4403 = t1523 * t1146;
    let t4410 = t106 * t454;
    let t4411 = t3169 * t1550;
    let t4434 = t3183 * t446;
    let t4435 = t4434 * t2667;
    (t4403, t4410, t4411, t4434, t4435)
}
