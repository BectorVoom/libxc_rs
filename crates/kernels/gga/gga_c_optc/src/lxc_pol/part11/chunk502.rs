//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 502/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk502<F: Float>(t1347: F, t809: F, t1359: F, t828: F, t1366: F, t2476: F, t241: F) -> (F, F, F, F) {
    let t3716 = t1347 * t809;
    let t3754 = t1359 * t828;
    let t3780 = t1366 * t2476;
    let t3788 = t241 * t1359;
    (t3716, t3754, t3780, t3788)
}
