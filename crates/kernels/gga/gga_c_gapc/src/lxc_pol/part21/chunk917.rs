//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 917/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk917<F: Float>(t11207: F, t8351: F, t147: F, t1509: F, t19: F, t3155: F, t681: F, t2920: F, t423: F, t1338: F, t3156: F, t1403: F, t3116: F, t1457: F, t632: F, t1266: F) -> (F, F, F, F, F, F, F) {
    let t25117 = t8351 * t11207;
    let t25127 = t3155 * t681 * t1509 * t19 * t147;
    let t25176 = t2920 * t423;
    let t25202 = t3156 * t1338 * t19 * t147;
    let t25382 = t3116 * t1403 * t19 * t147;
    let t25514 = t632 * t1457;
    let t25526 = t1266 * t1457;
    (t25117, t25127, t25176, t25202, t25382, t25514, t25526)
}
