//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 701/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk701<F: Float>(t1444: F, t604: F, t1181: F, t7575: F, t1449: F, t7351: F, t7564: F, t1541: F, t7647: F, t1456: F, t2001: F, t1165: F, t1421: F, t7493: F, t3220: F, t56: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8445 = t604 * t1444;
    let t8446 = t1181 * t8445;
    let t8447 = t7575 * t8446;
    let t8449 = t7351 * t1449;
    let t8450 = t1181 * t8449;
    let t8451 = t7564 * t8450;
    let t8453 = t7647 * t1541;
    let t8455 = t2001 * t1456;
    let t8458 = t1165 * t604 * t1421;
    let t8459 = t7493 * t8458;
    let t8461 = t56 * t3220;
    (t8446, t8447, t8450, t8451, t8453, t8455, t8458, t8459, t8461)
}
