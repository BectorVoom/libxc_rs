//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 287/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk287<F: Float>(t1027: F, t1135: F, t469: F, t454: F, t19: F, t463: F) -> (F, F, F, F, F) {
    let t1136 = t1135 * t1027;
    let t1145 = t469 * t469;
    let t1146 = F::new(1.0) / t1145;
    let t1147 = t454 * t1146;
    let t1148 = t19 * t463;
    (t1136, t1145, t1146, t1147, t1148)
}
