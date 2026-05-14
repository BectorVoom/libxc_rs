//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 710/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk710<F: Float>(t1: F, t9117: F, t4456: F, t8193: F, t3107: F, t935: F, t3102: F, t8113: F) -> (F, F, F, F, F) {
    let t9118 = t9117 * t1;
    let t9122 = t4456 * t8193;
    let t9123 = t3107 * t935;
    let t9124 = t9123 * t1;
    let t9128 = t3102 * t8113;
    (t9118, t9122, t9123, t9124, t9128)
}
