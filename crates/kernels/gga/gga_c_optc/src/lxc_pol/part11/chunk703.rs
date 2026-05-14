//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 703/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk703<F: Float>(t8639: F, t8642: F, t3057: F, t411: F) -> (F, F, F) {
    let t8727 = 0.93932222222222222223e0 * t8639;
    let t8728 = 0.36793333333333333333e0 * t8642;
    let t8749 = 1.0 / t3057 / t411;
    (t8727, t8728, t8749)
}
