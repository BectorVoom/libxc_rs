//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 636/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk636<F: Float>(t1141: F, t1146: F, t1145: F, t469: F, t454: F, t1182: F) -> (F, F, F, F) {
    let t3164 = t1141 * t1146;
    let t3169 = F::cast_from(1.0_f64) / t1145 / t469;
    let t3170 = t454 * t3169;
    let t3171 = t1182 * t1182;
    (t3164, t3169, t3170, t3171)
}
