//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 649/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk649<F: Float>(t1431: F, t442: F, t128: F, t1631: F, t1463: F, t1457: F, t431: F) -> (F, F, F, F) {
    let t4687 = t1431 * t442;
    let t4780 = t1631 * t128;
    let t4855 = t1463 * t442;
    let t4864 = t431 * t1457;
    (t4687, t4780, t4855, t4864)
}
