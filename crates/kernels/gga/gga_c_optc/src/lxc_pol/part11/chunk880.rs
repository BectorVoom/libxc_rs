//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 880/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk880<F: Float>(t4111: F, t5133: F, t1045: F, t17380: F, t17340: F, t2869: F, t25: F) -> (F, F, F, F) {
    let t17401 = t4111 * t5133;
    let t17403 = t1045 * t17380;
    let t17405 = t2869 * t17340;
    let t17406 = t25 * t17405;
    (t17401, t17403, t17405, t17406)
}
