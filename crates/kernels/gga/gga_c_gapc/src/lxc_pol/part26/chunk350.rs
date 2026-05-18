//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 350/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk350<F: Float>(t431: F, t515: F, t126: F, t514: F, t144: F, t190: F, t200: F, t442: F, t583: F) -> (F, F, F, F, F, F) {
    let t1572 = t431 * t515;
    let t1573 = t1572 * t126;
    let t1574 = t514 * t1573;
    let t1575 = t190 * t144;
    let t1576 = t1575 * t200;
    let t1577 = t583 * t442;
    (t1572, t1573, t1574, t1575, t1576, t1577)
}
