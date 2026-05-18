//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 991/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk991<F: Float>(t1441: F, t3228: F, t1418: F, t1347: F, t1005: F, t5251: F, t5232: F, t997: F, t1588: F, t3237: F, t1106: F, t372: F) -> (F, F, F, F, F, F, F) {
    let t16440 = t3228 * t1441;
    let t16442 = t3228 * t1418;
    let t16444 = t3228 * t1347;
    let t16446 = t1005 * t5251;
    let t16498 = t997 * t5232;
    let t16500 = t3237 * t1588;
    let t16507 = t1106 * t372;
    (t16440, t16442, t16444, t16446, t16498, t16500, t16507)
}
