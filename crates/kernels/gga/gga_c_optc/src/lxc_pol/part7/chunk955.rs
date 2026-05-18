//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 955/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk955<F: Float>(t3182: F, t426: F, t8193: F, t8915: F, t935: F, t1: F, t9103: F, t4456: F, t3107: F, t3102: F, t8113: F, t3104: F, t8914: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9114 = t3182 * sigma2;
    let t9115 = t9114 * t426;
    let t9116 = t9115 * t8193;
    let t9117 = t8915 * t935;
    let t9118 = t9117 * t1;
    let t9119 = t9103 * t9118;
    let t9122 = t4456 * t8193;
    let t9123 = t3107 * t935;
    let t9124 = t9123 * t1;
    let t9125 = t9103 * t9124;
    let t9128 = t3102 * t8113;
    let t9129 = t3104 * t8914;
    (t9114, t9115, t9116, t9117, t9118, t9119, t9122, t9123, t9124, t9125, t9128, t9129)
}
