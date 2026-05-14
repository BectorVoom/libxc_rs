//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 249/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk249<F: Float>(t103: F, t193: F, t197: F, t745: F, t102: F, t195: F, t616: F) -> (F, F, F) {
    let t749 = 100.0 / 27.0 * t193 * t745 * t103 * t197;
    let t750 = t195 * t102;
    let t751 = t197 * t616;
    let t755 = t749 - 25.0 / 9.0 * t193 * t750 * t751;
    (t750, t751, t755)
}
