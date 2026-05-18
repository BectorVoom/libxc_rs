//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 986/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk986<F: Float>(t11970: F, t4356: F, t1027: F, t8446: F, t10990: F, t465: F, t438: F, t553: F, t1123: F, t1028: F, t4328: F, t3187: F, t935: F) -> (F, F, F, F, F, F) {
    let t11971 = t11970 * t4356;
    let t11975 = t8446 * t1027;
    let t11982 = t465 * t10990;
    let t11984 = t438 * t553;
    let t11985 = t1123 * t11984;
    let t12004 = t4328 * t1028;
    let t12029 = t3187 * t935;
    (t11971, t11975, t11982, t11985, t12004, t12029)
}
